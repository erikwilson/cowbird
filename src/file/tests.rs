use super::*;
use std::fs::read_to_string;
use tempfile::{NamedTempFile, TempDir};

#[test]
fn test_create() {
    let dir = &TempDir::new().unwrap();
    let file = dir.path().join("create-test");
    assert_eq!(file.is_file(), false);
    create(file.to_str().unwrap());
    assert_eq!(file.is_file(), true);
}

#[test]
fn test_modify() {
    let tmp_file = &NamedTempFile::new().unwrap();
    assert_eq!(tmp_file.path().is_file(), true);
    let file = tmp_file.path().to_str().unwrap();
    assert_eq!(read_to_string(file).unwrap(), "");
    modify(file, "ok", &0);
    assert_eq!(read_to_string(file).unwrap(), "ok");
    modify(file, " ok", &2);
    assert_eq!(read_to_string(file).unwrap(), "ok ok");
}

#[test]
fn test_delete() {
    let tmp_file = &NamedTempFile::new().unwrap();
    assert_eq!(tmp_file.path().is_file(), true);
    delete(tmp_file.path().to_str().unwrap());
    assert_eq!(tmp_file.path().is_file(), false);
}
