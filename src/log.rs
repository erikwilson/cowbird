use slog::{Drain, FilterLevel, Logger};
use slog_envlogger::LogBuilder;
use slog_json::Json;
use std::io::Write;
use std::sync::Mutex;

lazy_static! {
    pub static ref LOGGER: Logger = {
        let pkg_info = o!("name" => env!["CARGO_PKG_NAME"], "version" => env!("CARGO_PKG_VERSION"));

        let output = match &*get_log_file() {
            "-" => Box::new(std::io::stdout()) as Box<dyn Write + Send>,
            path => Box::new(
                std::fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(true)
                    .open(path)
                    .expect("failed to open log file"),
            ) as Box<dyn Write + Send>,
        };

        let mut builder = LogBuilder::new(Json::default(output)).filter(None, FilterLevel::Info);
        if let Ok(s) = std::env::var("LOG_LEVEL") {
            builder = builder.parse(&s);
        }

        Logger::root(Mutex::new(builder.build()).fuse(), pkg_info)
    };
    static ref LOG_FILE: Mutex<String> = Mutex::new("-".to_string());
}

pub fn set_log_file(log_file: String) {
    *LOG_FILE.lock().unwrap() = log_file;
}

pub fn get_log_file() -> String {
    return LOG_FILE.lock().unwrap().clone();
}
