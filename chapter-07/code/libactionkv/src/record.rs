use crate::{KeydirEntry, StoreError};
use std::io::{self, ErrorKind};
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) enum Record {
    Upsert {
        key: String,
        value: String,
        timestamp: u64,
    },
    Delete {
        key: String,
        timestamp: u64,
    },
}

impl Record {
    const HEADER_SIZE: usize = 17;
    const KIND_UPSERT: u8 = 1;
    const KIND_DELETE: u8 = 2;

    pub(crate) fn upsert(key: &str, value: &str) -> Self {
        Self::Upsert {
            key: key.to_string(),
            value: value.to_string(),
            timestamp: timestamp(),
        }
    }

    pub(crate) fn delete(key: &str) -> Self {
        Self::Delete {
            key: key.to_string(),
            timestamp: timestamp(),
        }
    }

    pub(crate) fn encode(&self) -> Result<Vec<u8>, StoreError> {
        let key = self.key().as_bytes();
        let value = self.value().unwrap_or_default().as_bytes();
        let key_size = u32::try_from(key.len()).map_err(|_| StoreError::InvalidRecord)?;
        let value_size = u32::try_from(value.len()).map_err(|_| StoreError::InvalidRecord)?;

        let mut bytes = Vec::with_capacity(Self::HEADER_SIZE + key.len() + value.len());
        bytes.push(self.kind());
        bytes.extend_from_slice(&self.timestamp().to_le_bytes());
        bytes.extend_from_slice(&key_size.to_le_bytes());
        bytes.extend_from_slice(&value_size.to_le_bytes());
        bytes.extend_from_slice(key);
        bytes.extend_from_slice(value);

        Ok(bytes)
    }

    pub(crate) fn decode_at(
        bytes: &[u8],
        offset: usize,
    ) -> io::Result<Option<(DecodedRecord, usize)>> {
        if offset == bytes.len() {
            return Ok(None);
        }

        if bytes.len() - offset < Self::HEADER_SIZE {
            return Err(invalid_data("incomplete record header"));
        }

        let header = &bytes[offset..offset + Self::HEADER_SIZE];
        let kind = header[0];
        let timestamp = u64::from_le_bytes(header[1..9].try_into().unwrap());
        let key_size = u32::from_le_bytes(header[9..13].try_into().unwrap()) as usize;
        let value_size = u32::from_le_bytes(header[13..17].try_into().unwrap()) as usize;
        let body_offset = offset + Self::HEADER_SIZE;
        let value_offset = body_offset + key_size;
        let next_offset = value_offset
            .checked_add(value_size)
            .ok_or_else(|| invalid_data("record size overflow"))?;

        if next_offset > bytes.len() {
            return Err(invalid_data("incomplete record body"));
        }

        let key = String::from_utf8(bytes[body_offset..value_offset].to_vec())
            .map_err(|_| invalid_data("key is not valid utf-8"))?;

        let record = match kind {
            Self::KIND_UPSERT => DecodedRecord::Upsert {
                key,
                entry: KeydirEntry {
                    value_offset: value_offset as u64,
                    value_size: value_size as u32,
                    timestamp,
                },
            },
            Self::KIND_DELETE if value_size == 0 => DecodedRecord::Delete { key },
            Self::KIND_DELETE => return Err(invalid_data("delete record contains a value")),
            _ => return Err(invalid_data("unknown record kind")),
        };

        Ok(Some((record, next_offset)))
    }

    pub(crate) fn keydir_entry(&self, record_offset: u64) -> KeydirEntry {
        let key_size = self.key().len() as u64;
        let value_size = self.value().map_or(0, str::len) as u32;

        KeydirEntry {
            value_offset: record_offset + Self::HEADER_SIZE as u64 + key_size,
            value_size,
            timestamp: self.timestamp(),
        }
    }

    fn kind(&self) -> u8 {
        match self {
            Self::Upsert { .. } => Self::KIND_UPSERT,
            Self::Delete { .. } => Self::KIND_DELETE,
        }
    }

    fn key(&self) -> &str {
        match self {
            Self::Upsert { key, .. } | Self::Delete { key, .. } => key,
        }
    }

    fn value(&self) -> Option<&str> {
        match self {
            Self::Upsert { value, .. } => Some(value),
            Self::Delete { .. } => None,
        }
    }

    fn timestamp(&self) -> u64 {
        match self {
            Self::Upsert { timestamp, .. } | Self::Delete { timestamp, .. } => *timestamp,
        }
    }
}

pub(crate) enum DecodedRecord {
    Upsert { key: String, entry: KeydirEntry },
    Delete { key: String },
}

fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs())
}

fn invalid_data(message: &'static str) -> io::Error {
    io::Error::new(ErrorKind::InvalidData, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds record bytes from a readable fixture string.
    ///
    /// Input:  "upsert ts=7 key=my-key value=my-value"
    /// Output: [1, 7, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 8, 0, 0, 0, ...]
    fn encoded_record(record: &str) -> Vec<u8> {
        let mut parts = record.split_whitespace();
        let kind = parts.next().unwrap();
        let timestamp = field(parts.next().unwrap(), "ts=").parse::<u64>().unwrap();
        let key = field(parts.next().unwrap(), "key=").as_bytes();
        let value = match parts.next() {
            Some(value) => field(value, "value=").as_bytes(),
            None => &[],
        };

        let kind = match kind {
            "upsert" => Record::KIND_UPSERT,
            "delete" => Record::KIND_DELETE,
            other => panic!("unknown record kind {other}"),
        };

        let mut bytes = Vec::new();
        bytes.push(kind);
        bytes.extend_from_slice(&timestamp.to_le_bytes());
        bytes.extend_from_slice(&(key.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&(value.len() as u32).to_le_bytes());
        bytes.extend_from_slice(key);
        bytes.extend_from_slice(value);
        bytes
    }

    fn field<'a>(value: &'a str, prefix: &str) -> &'a str {
        value.strip_prefix(prefix).unwrap()
    }

    fn decoded_summary(bytes: &[u8], offset: usize) -> io::Result<Option<(String, usize)>> {
        Record::decode_at(bytes, offset).map(|decoded| {
            decoded.map(|(record, next_offset)| {
                let summary = match record {
                    DecodedRecord::Upsert { key, entry } => format!(
                        "upsert key={key} value_offset={} value_size={} ts={}",
                        entry.value_offset, entry.value_size, entry.timestamp
                    ),
                    DecodedRecord::Delete { key } => format!("delete key={key}"),
                };

                (summary, next_offset)
            })
        })
    }

    #[test]
    fn encode_writes_upsert_record_as_kind_timestamp_sizes_key_value() {
        let record = Record::Upsert {
            key: "my-key".to_string(),
            value: "my-value".to_string(),
            timestamp: 7,
        };

        assert_eq!(
            encoded_record("upsert ts=7 key=my-key value=my-value"),
            record.encode().unwrap()
        );
    }

    #[test]
    fn encode_writes_delete_record_as_kind_timestamp_key_size_zero_value_size_key() {
        let record = Record::Delete {
            key: "my-key".to_string(),
            timestamp: 7,
        };

        assert_eq!(
            encoded_record("delete ts=7 key=my-key"),
            record.encode().unwrap()
        );
    }

    #[test]
    fn decode_reads_upsert_record_into_keydir_entry() {
        let bytes = encoded_record("upsert ts=7 key=my-key value=my-value");

        assert_eq!(
            Some((
                "upsert key=my-key value_offset=23 value_size=8 ts=7".to_string(),
                31,
            )),
            decoded_summary(&bytes, 0).unwrap()
        );
    }

    #[test]
    fn decode_reads_delete_record_into_tombstone() {
        let bytes = encoded_record("delete ts=7 key=my-key");

        assert_eq!(
            Some(("delete key=my-key".to_string(), 23)),
            decoded_summary(&bytes, 0).unwrap()
        );
    }

    #[test]
    fn decode_reads_record_at_offset_and_returns_next_offset() {
        let first = encoded_record("delete ts=1 key=old-key");
        let second = encoded_record("upsert ts=2 key=new-key value=new-value");
        let second_offset = first.len();
        let mut bytes = first;
        bytes.extend_from_slice(&second);

        assert_eq!(
            Some((
                "upsert key=new-key value_offset=48 value_size=9 ts=2".to_string(),
                57,
            )),
            decoded_summary(&bytes, second_offset).unwrap()
        );
    }

    #[test]
    fn decode_returns_none_at_end_of_file() {
        let bytes = encoded_record("upsert ts=7 key=my-key value=my-value");

        assert_eq!(None, decoded_summary(&bytes, bytes.len()).unwrap());
    }

    #[test]
    fn decode_rejects_incomplete_header() {
        let bytes = vec![Record::KIND_UPSERT];

        let error = decoded_summary(&bytes, 0).unwrap_err();

        assert_eq!(ErrorKind::InvalidData, error.kind());
        assert_eq!("incomplete record header", error.to_string());
    }

    #[test]
    fn decode_rejects_incomplete_body() {
        let mut bytes = encoded_record("upsert ts=7 key=my-key value=my-value");
        bytes.pop();

        let error = decoded_summary(&bytes, 0).unwrap_err();

        assert_eq!(ErrorKind::InvalidData, error.kind());
        assert_eq!("incomplete record body", error.to_string());
    }

    #[test]
    fn decode_rejects_unknown_record_kind() {
        let mut bytes = encoded_record("upsert ts=7 key=my-key value=my-value");
        bytes[0] = 99;

        let error = decoded_summary(&bytes, 0).unwrap_err();

        assert_eq!(ErrorKind::InvalidData, error.kind());
        assert_eq!("unknown record kind", error.to_string());
    }

    #[test]
    fn decode_rejects_delete_record_with_value() {
        let bytes = encoded_record("delete ts=7 key=my-key value=unexpected");

        let error = decoded_summary(&bytes, 0).unwrap_err();

        assert_eq!(ErrorKind::InvalidData, error.kind());
        assert_eq!("delete record contains a value", error.to_string());
    }
}
