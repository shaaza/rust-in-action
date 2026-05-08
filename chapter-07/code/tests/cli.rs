use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::NamedTempFile;

/// Builds an actionkv command from a readable CLI string.
///
/// Input:  "actionkv filepath insert my-key my-value", file
/// Output: Command::cargo_bin("actionkv").arg(file.path()).args(["insert", "my-key", "my-value"])
fn actionkv(command: &str, file: &NamedTempFile) -> Command {
    let mut parts = command.split_whitespace();

    assert_eq!(Some("actionkv"), parts.next());

    let mut cmd = Command::cargo_bin("actionkv").unwrap();

    for part in parts {
        if part == "filepath" {
            cmd.arg(file.path());
        } else {
            cmd.arg(part);
        }
    }

    cmd
}

// get

/// Runs get with a key.
#[test]
fn get_accepts_key() {
    let file = NamedTempFile::new().unwrap();

    actionkv("actionkv filepath get my-key", &file)
        .assert()
        .success()
        .stdout(predicate::str::contains(r#"my-key not found in"#));
}

/// Rejects get without a key.
#[test]
fn get_requires_key() {
    let file = NamedTempFile::new().unwrap();

    actionkv("actionkv filepath get", &file)
        .assert()
        .failure()
        .stderr(predicate::str::contains("<KEY>"));
}

// insert

/// Runs insert with a key and value.
#[test]
fn insert_accepts_key_and_value() {
    let file = NamedTempFile::new().unwrap();

    actionkv("actionkv filepath insert my-key my-value", &file)
        .assert()
        .success()
        .stdout(predicate::str::contains("insert my-key=my-value into"));
}

/// Rejects insert without a value.
#[test]
fn insert_requires_value() {
    let file = NamedTempFile::new().unwrap();

    actionkv("actionkv filepath insert my-key", &file)
        .assert()
        .failure()
        .stderr(predicate::str::contains("<VALUE>"));
}

// update

/// Rejects update when the key does not exist.
#[test]
fn update_rejects_missing_key() {
    let file = NamedTempFile::new().unwrap();

    actionkv("actionkv filepath update my-key my-value", &file)
        .assert()
        .failure()
        .stderr(predicate::str::contains("my-key"));
}

// delete

/// Rejects delete when the key does not exist.
#[test]
fn delete_rejects_missing_key() {
    let file = NamedTempFile::new().unwrap();

    actionkv("actionkv filepath delete my-key", &file)
        .assert()
        .failure()
        .stderr(predicate::str::contains("my-key"));
}
