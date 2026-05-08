mod data_file;

use data_file::DataFile;
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub trait Store {
    fn get(&mut self, key: &str) -> Result<Option<String>, StoreError>;
    fn insert(&mut self, key: &str, value: &str) -> Result<(), StoreError>;
    fn update(&mut self, key: &str, value: &str) -> Result<(), StoreError>;
    fn delete(&mut self, key: &str) -> Result<(), StoreError>;
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum StoreError {
    #[error("key {key} already exists in file {}", filepath.display())]
    KeyAlreadyExists { key: String, filepath: PathBuf },

    #[error("key {key} not found in file {}", filepath.display())]
    KeyNotFound { key: String, filepath: PathBuf },

    #[error("failed to write to file {}: {message}", filepath.display())]
    WriteFailed {
        filepath: PathBuf,
        message: String,
    },

    #[error("failed to read from file {}: {message}", filepath.display())]
    ReadFailed {
        filepath: PathBuf,
        message: String,
    },

    #[error("keys and values cannot contain tabs or newlines")]
    InvalidRecord,
}

pub struct KVStore {
    filepath: PathBuf,
    file: DataFile,
    index: HashMap<String, u64>,
}

impl KVStore {
    pub fn open(filepath: PathBuf) -> io::Result<Self> {
        let mut index = HashMap::new();

        let file = DataFile::open(&filepath)?;

        file.scan_lines(|offset, line| {
            match Record::parse(&line) {
                Some(Record::Upsert { key, .. }) => {
                    index.insert(key, offset);
                }
                Some(Record::Delete { key }) => {
                    index.remove(&key);
                }
                None => {}
            }
        })?;

        Ok(Self {
            filepath,
            file,
            index,
        })
    }

    pub fn filepath(&self) -> &Path {
        &self.filepath
    }

    fn append(&mut self, record: Record) -> Result<u64, StoreError> {
        self.file.append(record.to_line()?)
    }

    fn read_at(&mut self, offset: u64) -> Result<Option<Record>, StoreError> {
        Ok(Record::parse(&self.file.read_at(offset)?))
    }
}

enum Record {
    Upsert { key: String, value: String },
    Delete { key: String },
}

impl Record {
    fn parse(line: &str) -> Option<Self> {
        let mut parts = line.splitn(3, '\t');

        match (parts.next(), parts.next(), parts.next()) {
            (Some("set"), Some(key), Some(value)) => Some(Self::Upsert {
                key: key.to_string(),
                value: value.to_string(),
            }),
            (Some("delete"), Some(key), None) => Some(Self::Delete {
                key: key.to_string(),
            }),
            _ => None,
        }
    }

    fn to_line(&self) -> Result<String, StoreError> {
        match self {
            Record::Upsert { key, value } => {
                validate_field(key)?;
                validate_field(value)?;
                Ok(format!("set\t{key}\t{value}"))
            }
            Record::Delete { key } => {
                validate_field(key)?;
                Ok(format!("delete\t{key}"))
            }
        }
    }
}

fn validate_field(field: &str) -> Result<(), StoreError> {
    if field.contains(['\t', '\n', '\r']) {
        return Err(StoreError::InvalidRecord);
    }

    Ok(())
}

impl Store for KVStore {
    fn get(&mut self, key: &str) -> Result<Option<String>, StoreError> {
        let Some(offset) = self.index.get(key).copied() else {
            return Ok(None);
        };

        match self.read_at(offset)? {
            Some(Record::Upsert {
                key: record_key,
                value,
            }) if record_key == key => Ok(Some(value)),
            _ => Ok(None),
        }
    }

    fn insert(&mut self, key: &str, value: &str) -> Result<(), StoreError> {
        if self.index.contains_key(key) {
            return Err(StoreError::KeyAlreadyExists {
                key: key.to_string(),
                filepath: self.filepath.clone(),
            });
        }

        // Record owns strings, so writes clone the caller's key/value once.
        let offset = self.append(Record::Upsert {
            key: key.to_string(),
            value: value.to_string(),
        })?;
        self.index.insert(key.to_string(), offset);
        Ok(())
    }

    fn update(&mut self, key: &str, value: &str) -> Result<(), StoreError> {
        if !self.index.contains_key(key) {
            return Err(StoreError::KeyNotFound {
                key: key.to_string(),
                filepath: self.filepath.clone(),
            });
        }

        // Record owns strings, so writes clone the caller's key/value once.
        let offset = self.append(Record::Upsert {
            key: key.to_string(),
            value: value.to_string(),
        })?;
        self.index.insert(key.to_string(), offset);
        Ok(())
    }

    fn delete(&mut self, key: &str) -> Result<(), StoreError> {
        match self.index.contains_key(key) {
            true => {
                // Record owns strings, so writes clone the caller's key once.
                self.append(Record::Delete {
                    key: key.to_string(),
                })?;
                self.index.remove(key);
                Ok(())
            }
            false => Err(StoreError::KeyNotFound {
                key: key.to_string(),
                filepath: self.filepath.clone(),
            }),
        }
    }
}
