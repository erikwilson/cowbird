use serde_json::Value;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::process::Command;
use substring::Substring;
use tempfile::NamedTempFile;

// Needs a unix system with user-land

#[test]
#[cfg(target_family = "unix")]
#[ignore] // skip for code coverage, run with: cargo test -- --ignored
fn test_example_script() {
    let _listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    let output = Command::new("./examples/test.yaml").output().unwrap();
    let data = String::from_utf8(output.stdout).unwrap();
    let logs = String::from_utf8(output.stderr).unwrap();
    verify(data, logs);
}

#[test]
#[cfg(target_family = "unix")]
fn test_script_api() {
    let _listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    let log_file_tmp = NamedTempFile::new().unwrap();
    let log_file = log_file_tmp.path().to_str().unwrap();
    cowbird::log::set_log_file(log_file.to_string());
    let stdout_file_tmp = NamedTempFile::new().unwrap();
    let stdout_file = stdout_file_tmp.path().to_str().unwrap();
    let guard = stdio_override::StdoutOverride::override_file(stdout_file).unwrap();
    cowbird::util::script("./examples/test.yaml");
    drop(guard);
    let data = fs::read_to_string(stdout_file).unwrap();
    let logs = fs::read_to_string(log_file).unwrap();
    verify(data, logs);
}

// verify log data for needed fields
fn verify(data: String, logs: String) {
    assert_eq!(data, "\nok\n\n", "stderr: {}", logs);
    for line in logs.split('\n') {
        if line.substring(0, 1) == "{" {
            let _ = writeln!(std::io::stdout(), "{}", line);
            let v: Value = serde_json::from_str(line).unwrap();
            assert_eq!(v["msg"], "ok");
            assert!(!v["ts"].is_null());
            assert!(!v["level"].is_null());
            assert!(!v["name"].is_null());
            assert!(!v["version"].is_null());
            assert!(!v["username"].is_null());
            assert!(!v["cmd_line"].is_null());
            assert!(!v["pid"].is_null());
            assert!(!v["type"].is_null());
            assert!(!v["cmd"].is_null());
            let cmd = &*v["cmd"].as_str().unwrap();
            match cmd {
                "start" => {
                    assert!(!v["start_ts"].is_null());
                    assert!(!v["exec"].is_null());
                    assert!(!v["args"].is_null());
                }
                "create" => {
                    assert!(!v["file"].is_null());
                }
                "modify" => {
                    assert!(!v["file"].is_null());
                    assert!(!v["offset"].is_null());
                    assert!(!v["size"].is_null());
                }
                "delete" => {
                    assert!(!v["file"].is_null());
                }
                "send" => {
                    assert!(!v["source"].is_null());
                    assert!(!v["dest"].is_null());
                    assert!(!v["proto"].is_null());
                    assert!(!v["size"].is_null());
                }
                _ => panic!("unknown cmd: {}", cmd),
            }
        };
    }
}
