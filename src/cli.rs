use crate::{cmd, log};
use clap::{AppSettings, Parser};

#[derive(Parser)]
#[clap(author = "Erik Wilson <erik.e.wilson@gmail.com>")]
#[clap(name = env!["CARGO_PKG_NAME"])]
#[clap(version = env!["CARGO_PKG_VERSION"])]
#[clap(about = "EDR tool", long_about = None)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(global_setting(AppSettings::DisableHelpSubcommand))]
pub struct Options {
    /// Log file location
    #[clap(short, long, default_value_t = concat!(env!["CARGO_PKG_NAME"],".log").to_string())]
    pub log: String,

    #[clap(subcommand)]
    pub command: cmd::Commands,
}

pub fn parse() -> Options {
    let opts = Options::parse();
    log::set_log_file(opts.log.clone());
    opts
}

pub fn run() {
    cmd::run(&parse().command)
}
