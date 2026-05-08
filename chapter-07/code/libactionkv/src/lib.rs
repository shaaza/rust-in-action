mod data_file;
mod record;

use data_file::DataFile;
use record::{DecodedRecord, Record};
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

    #[error("record is too large to store")]
    InvalidRecord,
}

pub struct KVStore {
    filepath: PathBuf,
    file: DataFile,
    index: HashMap<String, KeydirEntry>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct KeydirEntry {
    record_offset: u64,
    record_size: u64,
    value_offset: u64,
    value_size: u32,
    timestamp: u64,
}

impl KVStore {
    pub fn open(filepath: PathBuf) -> io::Result<Self> {
        let mut index = HashMap::new();

        let file = DataFile::open(&filepath)?;

        let mut offset = 0;
        let bytes = file.read_all()?;

        while let Some((record, next_offset)) = Record::decode_at(&bytes, offset)? {
            match record {
                DecodedRecord::Upsert { key, entry } => {
                    index.insert(key, entry);
                }
                DecodedRecord::Delete { key, .. } => {
                    index.remove(&key);
                }
            }
            offset = next_offset;
        }

        Ok(Self {
            filepath,
            file,
            index,
        })
    }

    pub fn filepath(&self) -> &Path {
        &self.filepath
    }

    fn append(&mut self, record: Record) -> Result<KeydirEntry, StoreError> {
        let bytes = record.encode()?;
        let offset = self.file.append(&bytes)?;
        Ok(record.keydir_entry(offset))
    }
}

impl Store for KVStore {
    fn get(&mut self, key: &str) -> Result<Option<String>, StoreError> {
        let Some(entry) = self.index.get(key).copied() else {
            return Ok(None);
        };

        let bytes = self
            .file
            .read_at(entry.record_offset, entry.record_size as usize)?;
        Record::decode_at(&bytes, 0).map_err(|source| StoreError::ReadFailed {
            filepath: self.filepath.clone(),
            message: source.to_string(),
        })?;

        let value_start = (entry.value_offset - entry.record_offset) as usize;
        let value_end = value_start + entry.value_size as usize;
        let value =
            String::from_utf8(bytes[value_start..value_end].to_vec()).map_err(|source| {
                StoreError::ReadFailed {
                    filepath: self.filepath.clone(),
                    message: source.to_string(),
                }
            })?;

        Ok(Some(value))
    }

    fn insert(&mut self, key: &str, value: &str) -> Result<(), StoreError> {
        if self.index.contains_key(key) {
            return Err(StoreError::KeyAlreadyExists {
                key: key.to_string(),
                filepath: self.filepath.clone(),
            });
        }

        let entry = self.append(Record::upsert(key, value))?;
        self.index.insert(key.to_string(), entry);
        Ok(())
    }

    fn update(&mut self, key: &str, value: &str) -> Result<(), StoreError> {
        if !self.index.contains_key(key) {
            return Err(StoreError::KeyNotFound {
                key: key.to_string(),
                filepath: self.filepath.clone(),
            });
        }

        let entry = self.append(Record::upsert(key, value))?;
        self.index.insert(key.to_string(), entry);
        Ok(())
    }

    fn delete(&mut self, key: &str) -> Result<(), StoreError> {
        match self.index.contains_key(key) {
            true => {
                self.append(Record::delete(key))?;
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
