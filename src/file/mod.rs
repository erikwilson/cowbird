//! File oriented commands

use crate::{log, util::binary_decode};
use positioned_io::WriteAt;
use std::fs;

#[cfg(test)]
mod tests;

lazy_static! {
    static ref LOGGER: slog::Logger = log::LOGGER.new(o!("type" => "file"));
}

/// Create a file
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

/// Modify a file given some data and offset
pub fn modify(file: &str, data: &str, offset: &u64) {
    let bytes: &[u8] = &*binary_decode(data);
    let logger = LOGGER.new(o!(
        "cmd" => "modify",
        "file" => file.to_string(),
        "size" => bytes.len(),
        "offset" => *offset,
    ));
    match fs::OpenOptions::new().write(true).open(file) {
        Ok(mut f) => match f.write_at(*offset, bytes) {
            Ok(_) => info!(logger, "ok"),
            Err(error) => error!(logger, "error: write: {}", error),
        },
        Err(error) => error!(logger, "error: open: {}", error),
    }
}

/// Delete a file
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
