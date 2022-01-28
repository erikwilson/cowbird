use crate::{file, log, network, process, util};
use clap::{Parser, Subcommand};
use network::Protocol;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Subcommand, Debug, PartialEq, Serialize, Deserialize)]
pub enum Commands {
    /// Starts a new process
    Start {
        // Executable to run
        exec: String,
        // Arguments for executable
        #[serde(default)]
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
        data: String,
        /// Byte offset for writing data
        #[clap(default_value_t)]
        #[serde(default)]
        offset: u64,
    },
    /// Delete a file
    Delete {
        /// File to delete
        file: String,
    },
    /// Send data over network
    Send {
        /// Destination address:port
        dest: SocketAddr,
        /// Bytes to write, decode hex if value starts with "0x"
        #[clap(default_value = "")]
        #[serde(default)]
        data: String,
        /// Network protocol to use
        #[clap(arg_enum, default_value_t)]
        #[serde(default)]
        proto: Protocol,
    },
    /// Run commands from YAML input
    Script {
        /// File to read
        #[clap(default_value = "-")]
        file: String,
    },
}

pub fn run(command: &Commands) {
    match command {
        Commands::Start { exec, args } => process::start(exec, args),

        Commands::Create { file } => file::create(file),

        Commands::Modify { file, data, offset } => file::modify(file, data, offset),

        Commands::Delete { file } => file::delete(file),

        Commands::Send { dest, data, proto } => network::send(dest, data, proto),

        Commands::Script { file } => util::script(file),
    }
}
