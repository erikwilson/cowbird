use super::*;
use crate::cmd;
use std::fs::File;
use std::io::prelude::*;
use tempfile::TempDir;

#[test]
fn test_script() {
    let dir = &TempDir::new().unwrap();
    let file = dir.path().join("proc-start-touch-test");
    let src: &str = &dir.path().join("touch.yaml").to_str().unwrap().to_string();
    let command = &cmd::Commands::Start {
        exec: String::from("touch"),
        args: vec![file.to_str().unwrap().to_string()],
    };
    let cmd_src = serde_yaml::to_string(command).unwrap();
    File::create(src).unwrap().write_all(cmd_src.as_bytes());
    assert!(!file.is_file());
    script(src);
    assert!(file.is_file());
}
