use super::*;
use tempfile::TempDir;

#[test]
fn test_start() {
    let dir = &TempDir::new().unwrap();
    let file = dir.path().join("proc-start-touch-test");
    assert_eq!(file.is_file(), false);
    start("touch", &[file.to_str().unwrap().to_string()]);
    assert_eq!(file.is_file(), true);
}
