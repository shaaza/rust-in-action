use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub trait Store {
    fn get(&self, key: &str) -> Option<&str>;
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
}

pub struct KVStore {
    filepath: PathBuf,
    data: HashMap<String, String>,
}

impl KVStore {
    pub fn open(filepath: PathBuf) -> Self {
        Self {
            filepath,
            data: HashMap::new(),
        }
    }

    pub fn filepath(&self) -> &Path {
        &self.filepath
    }
}

impl Store for KVStore {
    fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(String::as_str)
    }

    fn insert(&mut self, key: &str, value: &str) -> Result<(), StoreError> {
        if self.data.contains_key(key) {
            return Err(StoreError::KeyAlreadyExists {
                key: key.to_string(),
                filepath: self.filepath.clone(),
            });
        }

        self.data.insert(key.to_string(), value.to_string());
        Ok(())
    }

    fn update(&mut self, key: &str, value: &str) -> Result<(), StoreError> {
        if !self.data.contains_key(key) {
            return Err(StoreError::KeyNotFound {
                key: key.to_string(),
                filepath: self.filepath.clone(),
            });
        }

        self.data.insert(key.to_string(), value.to_string());
        Ok(())
    }

    fn delete(&mut self, key: &str) -> Result<(), StoreError> {
        match self.data.remove(key) {
            Some(_) => Ok(()),
            None => Err(StoreError::KeyNotFound {
                key: key.to_string(),
                filepath: self.filepath.clone(),
            }),
        }
    }
}
