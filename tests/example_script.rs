use serde_json::Value;
use std::io::prelude::*;
use std::net::TcpListener;
use std::process::Command;
use substring::Substring;

#[test]
fn test_example_script() {
    let _listener = &TcpListener::bind("127.0.0.1:8888").unwrap();
    let output = Command::new("./examples/test.yaml").output().unwrap();
    let data = String::from_utf8(output.stdout).unwrap();
    let logs = String::from_utf8(output.stderr).unwrap();
    assert_eq!(data, "\nok\n\n");
    for line in logs.split("\n") {
        if line.substring(0, 1) == "{" {
            let _ = write!(std::io::stdout(), "{}\n", line);
            let v: Value = serde_json::from_str(line).unwrap();
            assert_eq!(v["msg"], "ok");
            assert_eq!(v["ts"].is_null(), false);
            assert_eq!(v["level"].is_null(), false);
            assert_eq!(v["name"].is_null(), false);
            assert_eq!(v["version"].is_null(), false);
            assert_eq!(v["username"].is_null(), false);
            assert_eq!(v["cmd_line"].is_null(), false);
            assert_eq!(v["pid"].is_null(), false);
            assert_eq!(v["type"].is_null(), false);
            assert_eq!(v["cmd"].is_null(), false);
            let cmd = &*v["cmd"].as_str().unwrap();
            match cmd {
                "start" => {
                    assert_eq!(v["start_ts"].is_null(), false);
                    assert_eq!(v["exec"].is_null(), false);
                    assert_eq!(v["args"].is_null(), false);
                }
                "create" => {
                    assert_eq!(v["file"].is_null(), false);
                }
                "modify" => {
                    assert_eq!(v["file"].is_null(), false);
                    assert_eq!(v["offset"].is_null(), false);
                    assert_eq!(v["size"].is_null(), false);
                }
                "delete" => {
                    assert_eq!(v["file"].is_null(), false);
                }
                "send" => {
                    assert_eq!(v["source"].is_null(), false);
                    assert_eq!(v["dest"].is_null(), false);
                    assert_eq!(v["proto"].is_null(), false);
                    assert_eq!(v["size"].is_null(), false);
                }
                _ => assert!(false, "unknown cmd: {}", cmd),
            }
        };
    }
}
