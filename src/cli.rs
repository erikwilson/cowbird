use crate::{file, log, network, process, util};
use clap::{AppSettings, Parser, Subcommand};
use network::Protocol;
use std::net::SocketAddr;
use util::BinaryData;

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
    Start {
        // Executable to run
        executable: String,
        // Arguments for executable
        args: Vec<String>,
    },
    /// Create a file
    Create {
        /// File to create
        file: String,
    },
    /// Modify a file
    Modify {
        /// File to modify
        file: String,
        /// Bytes to write, decode hex if value starts with "0x"
        data: BinaryData,
        /// Byte offset for writing data
        #[clap(default_value_t = 0)]
        offset: u64,
    },
    /// Delete a file
    Delete {
        /// File to delete
        file: String,
    },
    /// Connect and send data over network
    Connect {
        /// Destination address:port
        destination: SocketAddr,
        /// Bytes to write, decode hex if value starts with "0x"
        #[clap(default_value = "")]
        data: BinaryData,
        /// Network protocol to use
        #[clap(default_value = "UDP")]
        protocol: Protocol,
    },
}

pub fn parse() -> Options {
    let opts = Options::parse();
    log::set_log_file(opts.log.clone());
    opts
}

pub fn run() {
    match &parse().command {
        Commands::Start { executable, args } => process::start(executable, args),

        Commands::Create { file } => file::create(file),

        Commands::Modify { file, data, offset } => file::modify(file, &*data.bytes, offset),

        Commands::Delete { file } => file::delete(file),

        Commands::Connect {
            destination,
            data,
            protocol,
        } => network::connect(protocol, destination, &*data.bytes),
    }
}
