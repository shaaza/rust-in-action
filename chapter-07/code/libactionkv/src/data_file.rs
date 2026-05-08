use crate::StoreError;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Seek, SeekFrom, Write};
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

    pub(crate) fn append(&mut self, line: String) -> Result<u64, StoreError> {
        let offset = self
            .file
            .seek(SeekFrom::End(0))
            .map_err(|source| self.write_error(source))?;

        writeln!(self.file, "{line}")
            .and_then(|_| self.file.flush())
            .map_err(|source| self.write_error(source))?;

        Ok(offset)
    }

    pub(crate) fn read_at(&mut self, offset: u64) -> Result<String, StoreError> {
        self.file
            .seek(SeekFrom::Start(offset))
            .map_err(|source| self.read_error(source))?;

        let mut line = String::new();
        let mut reader = BufReader::new(&mut self.file);

        reader
            .read_line(&mut line)
            .map_err(|source| self.read_error(source))?;

        Ok(line.trim_end_matches(['\r', '\n']).to_string())
    }

    pub(crate) fn scan_lines<F>(&self, mut visit: F) -> io::Result<()>
    where
        F: FnMut(u64, &str),
    {
        let mut reader = BufReader::new(File::open(&self.filepath)?);
        let mut offset = 0;
        let mut line = String::new();

        loop {
            line.clear();

            let bytes_read = reader.read_line(&mut line)?;
            if bytes_read == 0 {
                break;
            }

            visit(offset, line.trim_end_matches(['\r', '\n']));
            offset += bytes_read as u64;
        }

        Ok(())
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
