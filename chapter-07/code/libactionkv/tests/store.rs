use libactionkv::{KVStore, Store, StoreError};
use std::path::PathBuf;

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
    let store = KVStore::open(PathBuf::from("test.db")).unwrap();

    assert_eq!(None, store.get("my-key"));
}

#[test]
fn insert_stores_new_key() {
    let mut store = KVStore::open(PathBuf::from("test.db")).unwrap();

    assert_eq!(Ok(()), store.insert("my-key", "my-value"));
    assert_eq!(Some("my-value"), store.get("my-key"));
}

#[test]
fn insert_rejects_existing_key() {
    let mut store = KVStore::open(PathBuf::from("test.db")).unwrap();

    assert_eq!(Ok(()), store.insert("my-key", "my-value"));
    assert_eq!(
        Err(StoreError::KeyAlreadyExists {
            key: "my-key".to_string(),
            filepath: PathBuf::from("test.db"),
        }),
        store.insert("my-key", "other-value")
    );
}

#[test]
fn update_changes_existing_key() {
    let mut store = KVStore::open(PathBuf::from("test.db")).unwrap();

    assert_eq!(Ok(()), store.insert("my-key", "my-value"));
    assert_eq!(Ok(()), store.update("my-key", "new-value"));
    assert_eq!(Some("new-value"), store.get("my-key"));
}

#[test]
fn delete_removes_existing_key() {
    let mut store = KVStore::open(PathBuf::from("test.db")).unwrap();

    assert_eq!(Ok(()), store.insert("my-key", "my-value"));
    assert_eq!(Ok(()), store.delete("my-key"));
    assert_eq!(None, store.get("my-key"));
}

#[test]
fn update_rejects_missing_key() {
    let mut store = KVStore::open(PathBuf::from("test.db")).unwrap();

    assert_eq!(
        Err(StoreError::KeyNotFound {
            key: "my-key".to_string(),
            filepath: PathBuf::from("test.db"),
        }),
        store.update("my-key", "new-value")
    );
}

#[test]
fn delete_rejects_missing_key() {
    let mut store = KVStore::open(PathBuf::from("test.db")).unwrap();

    assert_eq!(
        Err(StoreError::KeyNotFound {
            key: "my-key".to_string(),
            filepath: PathBuf::from("test.db"),
        }),
        store.delete("my-key")
    );
}
