use crate::{file, log, network, process};
use clap::{AppSettings, Parser, Subcommand};

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
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Starts a new process
    Exec {
        executable: String,
        args: Vec<String>,
    },
    /// Create or modify a file
    Copy { source: String, target: String },
    /// Delete a file
    Delete { file: String },
    /// Network copy
    NetCat { source: String, destination: String },
}

pub fn parse() -> Options {
    let opts = Options::parse();
    log::set_log_file(opts.log.clone());
    opts
}

pub fn run() {
    match &parse().command {
        Commands::Exec { executable, args } => process::exec(executable, args),

        Commands::Copy { source, target } => file::copy(source, target),

        Commands::Delete { file } => file::delete(file),

        Commands::NetCat {
            source,
            destination,
        } => network::net_cat(source, destination),
    }
}
