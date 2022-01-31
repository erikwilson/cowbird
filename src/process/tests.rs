use super::*;
use tempfile::TempDir;

#[test]
fn test_start() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("proc-start-touch-test");
    assert!(!file.is_file());
    start("touch", &[file.to_str().unwrap().to_string()]);
    assert!(file.is_file());
}
