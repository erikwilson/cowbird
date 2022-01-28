use crate::{log, util::binary_decode};
use std::fs;
#[cfg(target_family = "unix")]
use std::os::unix::prelude::FileExt;
#[cfg(target_family = "windows")]
use std::os::windows::prelude::FileExt;

lazy_static! {
    static ref LOGGER: slog::Logger = log::LOGGER.new(o!("type" => "file"));
}

pub fn create(file: &str) {
    let logger = LOGGER.new(o!(
        "cmd" => "create",
        "file" => file.to_string(),
    ));
    match fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file)
    {
        Ok(_) => info!(logger, "ok"),
        Err(error) => error!(logger, "error: {}", error),
    }
}

pub fn modify(file: &str, data: &str, offset: &u64) {
    let bytes: &[u8] = &*binary_decode(data);
    let logger = LOGGER.new(o!(
        "cmd" => "modify",
        "file" => file.to_string(),
        "size" => bytes.len(),
        "offset" => *offset,
    ));
    match fs::OpenOptions::new().write(true).open(file) {
        Ok(f) => match f.write_at(bytes, *offset) {
            Ok(_) => info!(logger, "ok"),
            Err(error) => error!(logger, "error: write: {}", error),
        },
        Err(error) => error!(logger, "error: open: {}", error),
    }
}

pub fn delete(file: &str) {
    let logger = LOGGER.new(o!(
        "cmd" => "delete",
        "file" => file.to_string(),
    ));
    match fs::remove_file(file) {
        Ok(_) => info!(logger, "ok"),
        Err(error) => error!(logger, "error: {}", error),
    }
}
