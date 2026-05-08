use crate::StoreError;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

pub(crate) struct DataFile {
    filepath: PathBuf,
    file: File,
}

impl DataFile {
    pub(crate) fn open(filepath: &Path) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(filepath)?;

        Ok(Self {
            filepath: filepath.to_path_buf(),
            file,
        })
    }

    pub(crate) fn append(&mut self, bytes: &[u8]) -> Result<u64, StoreError> {
        let offset = self
            .file
            .seek(SeekFrom::End(0))
            .map_err(|source| self.write_error(source))?;

        self.file
            .write_all(bytes)
            .and_then(|_| self.file.flush())
            .map_err(|source| self.write_error(source))?;

        Ok(offset)
    }

    pub(crate) fn read_at(&mut self, offset: u64, size: usize) -> Result<Vec<u8>, StoreError> {
        self.file
            .seek(SeekFrom::Start(offset))
            .map_err(|source| self.read_error(source))?;

        let mut bytes = vec![0; size];

        self.file
            .read_exact(&mut bytes)
            .map_err(|source| self.read_error(source))?;

        Ok(bytes)
    }

    pub(crate) fn read_all(&self) -> io::Result<Vec<u8>> {
        std::fs::read(&self.filepath)
    }

    fn read_error(&self, source: io::Error) -> StoreError {
        StoreError::ReadFailed {
            filepath: self.filepath.clone(),
            message: source.to_string(),
        }
    }

    fn write_error(&self, source: io::Error) -> StoreError {
        StoreError::WriteFailed {
            filepath: self.filepath.clone(),
            message: source.to_string(),
        }
    }
}
