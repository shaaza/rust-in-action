use libactionkv::{KVStore, Store, StoreError};
use std::fs;
use std::path::PathBuf;

fn store_file() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let filepath = dir.path().join("store.db");

    (dir, filepath)
}

#[test]
fn open_creates_file_when_missing() {
    let dir = tempfile::tempdir().unwrap();
    let filepath = dir.path().join("store.db");

    assert!(!filepath.exists());

    let store = KVStore::open(filepath.clone()).unwrap();

    assert!(filepath.exists());
    assert_eq!(filepath.as_path(), store.filepath());
}

#[test]
fn open_accepts_existing_file() {
    let file = tempfile::NamedTempFile::new().unwrap();
    let filepath = file.path().to_path_buf();

    let store = KVStore::open(filepath.clone()).unwrap();

    assert_eq!(filepath.as_path(), store.filepath());
}

#[test]
fn open_rejects_missing_parent_directory() {
    let dir = tempfile::tempdir().unwrap();
    let filepath = dir.path().join("missing").join("store.db");

    let result = KVStore::open(filepath);

    assert!(result.is_err());
}

#[test]
fn get_returns_none_for_missing_key() {
    let (_dir, filepath) = store_file();
    let mut store = KVStore::open(filepath).unwrap();

    assert_eq!(Ok(None), store.get("my-key"));
}

#[test]
fn insert_stores_new_key() {
    let (_dir, filepath) = store_file();
    let mut store = KVStore::open(filepath).unwrap();

    assert_eq!(Ok(()), store.insert("my-key", "my-value"));
    assert_eq!(Ok(Some("my-value".to_string())), store.get("my-key"));
}

#[test]
fn insert_rejects_existing_key() {
    let (_dir, filepath) = store_file();
    let mut store = KVStore::open(filepath.clone()).unwrap();

    assert_eq!(Ok(()), store.insert("my-key", "my-value"));
    assert_eq!(
        Err(StoreError::KeyAlreadyExists {
            key: "my-key".to_string(),
            filepath,
        }),
        store.insert("my-key", "other-value")
    );
}

#[test]
fn update_changes_existing_key() {
    let (_dir, filepath) = store_file();
    let mut store = KVStore::open(filepath).unwrap();

    assert_eq!(Ok(()), store.insert("my-key", "my-value"));
    assert_eq!(Ok(()), store.update("my-key", "new-value"));
    assert_eq!(Ok(Some("new-value".to_string())), store.get("my-key"));
}

#[test]
fn delete_removes_existing_key() {
    let (_dir, filepath) = store_file();
    let mut store = KVStore::open(filepath).unwrap();

    assert_eq!(Ok(()), store.insert("my-key", "my-value"));
    assert_eq!(Ok(()), store.delete("my-key"));
    assert_eq!(Ok(None), store.get("my-key"));
}

#[test]
fn update_rejects_missing_key() {
    let (_dir, filepath) = store_file();
    let mut store = KVStore::open(filepath.clone()).unwrap();

    assert_eq!(
        Err(StoreError::KeyNotFound {
            key: "my-key".to_string(),
            filepath,
        }),
        store.update("my-key", "new-value")
    );
}

#[test]
fn delete_rejects_missing_key() {
    let (_dir, filepath) = store_file();
    let mut store = KVStore::open(filepath.clone()).unwrap();

    assert_eq!(
        Err(StoreError::KeyNotFound {
            key: "my-key".to_string(),
            filepath,
        }),
        store.delete("my-key")
    );
}

#[test]
fn open_loads_inserted_key_from_file() {
    let (_dir, filepath) = store_file();

    {
        let mut store = KVStore::open(filepath.clone()).unwrap();
        assert_eq!(Ok(()), store.insert("my-key", "my-value"));
    }

    let mut store = KVStore::open(filepath).unwrap();

    assert_eq!(Ok(Some("my-value".to_string())), store.get("my-key"));
}

#[test]
fn open_loads_latest_update_from_file() {
    let (_dir, filepath) = store_file();

    {
        let mut store = KVStore::open(filepath.clone()).unwrap();
        assert_eq!(Ok(()), store.insert("my-key", "my-value"));
        assert_eq!(Ok(()), store.update("my-key", "new-value"));
    }

    let mut store = KVStore::open(filepath).unwrap();

    assert_eq!(Ok(Some("new-value".to_string())), store.get("my-key"));
}

#[test]
fn open_loads_deleted_key_as_missing() {
    let (_dir, filepath) = store_file();

    {
        let mut store = KVStore::open(filepath.clone()).unwrap();
        assert_eq!(Ok(()), store.insert("my-key", "my-value"));
        assert_eq!(Ok(()), store.delete("my-key"));
    }

    let mut store = KVStore::open(filepath).unwrap();

    assert_eq!(Ok(None), store.get("my-key"));
}

#[test]
fn open_loads_records_from_file_with_crlf_line_endings() {
    let (_dir, filepath) = store_file();
    fs::write(&filepath, "set\tfirst\tone\r\nset\tsecond\ttwo\r\n").unwrap();

    let mut store = KVStore::open(filepath).unwrap();

    assert_eq!(Ok(Some("one".to_string())), store.get("first"));
    assert_eq!(Ok(Some("two".to_string())), store.get("second"));
}

#[test]
fn insert_rejects_key_and_value_that_cannot_be_stored_as_one_line() {
    let (_dir, filepath) = store_file();
    let mut store = KVStore::open(filepath).unwrap();

    assert_eq!(Err(StoreError::InvalidRecord), store.insert("my\tkey", "my-value"));
    assert_eq!(Err(StoreError::InvalidRecord), store.insert("my-key", "my\nvalue"));
}
