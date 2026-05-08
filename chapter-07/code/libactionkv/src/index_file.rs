use crate::KeydirEntry;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

const MAGIC: &[u8; 8] = b"AKVIDX1\0";

pub(crate) fn path_for(data_filepath: &Path) -> PathBuf {
    let mut path = OsString::from(data_filepath.as_os_str());
    path.push(".idx");
    PathBuf::from(path)
}

pub(crate) fn load(
    data_filepath: &Path,
    index_filepath: &Path,
) -> io::Result<Option<HashMap<String, KeydirEntry>>> {
    let bytes = match fs::read(index_filepath) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };

    let data_len = fs::metadata(data_filepath)?.len();
    let mut cursor = Cursor::new(&bytes);

    if cursor.take(MAGIC.len())? != MAGIC {
        return Ok(None);
    }

    let indexed_data_len = cursor.u64()?;
    if indexed_data_len != data_len {
        return Ok(None);
    }

    let entry_count = cursor.u64()?;
    let mut index = HashMap::with_capacity(entry_count as usize);

    for _ in 0..entry_count {
        let key_len = cursor.u32()? as usize;
        let key = cursor.string(key_len)?;
        let entry = KeydirEntry {
            record_offset: cursor.u64()?,
            record_size: cursor.u64()?,
            value_offset: cursor.u64()?,
            value_size: cursor.u32()?,
            timestamp: cursor.u64()?,
        };

        index.insert(key, entry);
    }

    if !cursor.is_finished() {
        return Ok(None);
    }

    Ok(Some(index))
}

pub(crate) fn save(
    data_filepath: &Path,
    index_filepath: &Path,
    index: &HashMap<String, KeydirEntry>,
) -> io::Result<()> {
    let data_len = fs::metadata(data_filepath)?.len();
    let mut bytes = Vec::new();

    bytes.extend_from_slice(MAGIC);
    bytes.extend_from_slice(&data_len.to_le_bytes());
    bytes.extend_from_slice(&(index.len() as u64).to_le_bytes());

    for (key, entry) in index {
        let key = key.as_bytes();
        let key_len = u32::try_from(key.len())
            .map_err(|_| io::Error::new(ErrorKind::InvalidInput, "index key is too large"))?;

        bytes.extend_from_slice(&key_len.to_le_bytes());
        bytes.extend_from_slice(key);
        bytes.extend_from_slice(&entry.record_offset.to_le_bytes());
        bytes.extend_from_slice(&entry.record_size.to_le_bytes());
        bytes.extend_from_slice(&entry.value_offset.to_le_bytes());
        bytes.extend_from_slice(&entry.value_size.to_le_bytes());
        bytes.extend_from_slice(&entry.timestamp.to_le_bytes());
    }

    let mut temp_filepath = OsString::from(index_filepath.as_os_str());
    temp_filepath.push(".tmp");
    let temp_filepath = PathBuf::from(temp_filepath);

    fs::write(&temp_filepath, bytes)?;
    fs::rename(temp_filepath, index_filepath)
}

struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn take(&mut self, size: usize) -> io::Result<&'a [u8]> {
        let end = self
            .offset
            .checked_add(size)
            .ok_or_else(|| invalid_data("index offset overflow"))?;

        if end > self.bytes.len() {
            return Err(invalid_data("incomplete index file"));
        }

        let bytes = &self.bytes[self.offset..end];
        self.offset = end;
        Ok(bytes)
    }

    fn u32(&mut self) -> io::Result<u32> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }

    fn u64(&mut self) -> io::Result<u64> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }

    fn string(&mut self, size: usize) -> io::Result<String> {
        String::from_utf8(self.take(size)?.to_vec())
            .map_err(|_| invalid_data("index key is not valid utf-8"))
    }

    fn is_finished(&self) -> bool {
        self.offset == self.bytes.len()
    }
}

fn invalid_data(message: &'static str) -> io::Error {
    io::Error::new(ErrorKind::InvalidData, message)
}
