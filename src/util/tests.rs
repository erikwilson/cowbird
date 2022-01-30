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
    let cmd_src = format!("{}", serde_yaml::to_string(command).unwrap());
    File::create(src).unwrap().write_all(cmd_src.as_bytes());
    assert_eq!(file.is_file(), false);
    script(src);
    assert_eq!(file.is_file(), true);
}
