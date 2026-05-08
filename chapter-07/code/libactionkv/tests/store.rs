use libactionkv::{KVStore, Store, StoreError};
use std::path::PathBuf;

#[test]
fn get_returns_none_for_missing_key() {
    let store = KVStore::open(PathBuf::from("test.db"));

    assert_eq!(None, store.get("my-key"));
}

#[test]
fn insert_stores_new_key() {
    let mut store = KVStore::open(PathBuf::from("test.db"));

    assert_eq!(Ok(()), store.insert("my-key", "my-value"));
    assert_eq!(Some("my-value"), store.get("my-key"));
}

#[test]
fn insert_rejects_existing_key() {
    let mut store = KVStore::open(PathBuf::from("test.db"));

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
    let mut store = KVStore::open(PathBuf::from("test.db"));

    assert_eq!(Ok(()), store.insert("my-key", "my-value"));
    assert_eq!(Ok(()), store.update("my-key", "new-value"));
    assert_eq!(Some("new-value"), store.get("my-key"));
}

#[test]
fn delete_removes_existing_key() {
    let mut store = KVStore::open(PathBuf::from("test.db"));

    assert_eq!(Ok(()), store.insert("my-key", "my-value"));
    assert_eq!(Ok(()), store.delete("my-key"));
    assert_eq!(None, store.get("my-key"));
}

#[test]
fn update_rejects_missing_key() {
    let mut store = KVStore::open(PathBuf::from("test.db"));

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
    let mut store = KVStore::open(PathBuf::from("test.db"));

    assert_eq!(
        Err(StoreError::KeyNotFound {
            key: "my-key".to_string(),
            filepath: PathBuf::from("test.db"),
        }),
        store.delete("my-key")
    );
}
