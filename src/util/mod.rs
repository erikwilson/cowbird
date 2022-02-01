//! Utility module for parsing user input and scripts

use crate::{cmd, log};
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::str::FromStr;
use substring::Substring;
use yaml_split::DocumentIterator;

#[cfg(test)]
mod tests;

lazy_static! {
    static ref LOGGER: slog::Logger = log::LOGGER.new(o!("type" => "util"));
}

/// Attempt to parse string as hex if starts with "0x", returns vector of bytes
pub fn binary_decode(data: &str) -> Vec<u8> {
    match data.substring(0, 2) {
        "0x" => match hex::decode(data[2..].as_bytes()) {
            Ok(bytes) => bytes,
            Err(error) => {
                warn!(LOGGER, "parse error: {}", error);
                data.as_bytes().to_vec()
            }
        },
        _ => data.as_bytes().to_vec(),
    }
}

/// Ingest a YAML file containing commands and run each,
/// passing file as "-" reads from stdin
pub fn script(file: &str) {
    let input = match file {
        "-" => Box::new(std::io::stdin()) as Box<dyn Read + Send>,
        path => Box::new(
            std::fs::OpenOptions::new()
                .read(true)
                .open(path)
                .expect("failed to open input"),
        ) as Box<dyn Read + Send>,
    };
    for doc in DocumentIterator::new(input) {
        cmd::run(&serde_yaml::from_str(&*doc.unwrap()).expect("failed to parse input"));
    }
}
