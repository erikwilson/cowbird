use crate::log;
use chrono::Local;
use std::process::Command;
use std::time::SystemTime;

lazy_static! {
    static ref LOGGER: slog::Logger = log::LOGGER.new(o!("type" => "process"));
}

pub fn start(exec: &str, args: &[String]) {
    let logger = LOGGER.new(o!(
        "cmd" => "start",
        "exec" => exec.to_string(),
        "args" => args.to_vec().join(" "),
        "start_ts" => Local::now().to_rfc3339(),
    ));
    debug!(logger, "start");
    match Command::new(exec).args(args).status() {
        Ok(status) => match status.success() {
            true => info!(logger, "ok"),
            false => warn!(logger, "{}", status),
        },
        Err(error) => error!(logger, "error: {}", error),
    }
}
