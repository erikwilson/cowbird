use crate::log;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use substring::Substring;

lazy_static! {
    static ref LOGGER: slog::Logger = log::LOGGER.new(o!("type" => "util"));
}

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
