//! Network oriented commands

use crate::{log, util::binary_decode};
use clap::ArgEnum;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::str;

#[cfg(test)]
mod tests;

lazy_static! {
    static ref LOGGER: slog::Logger = log::LOGGER.new(o!("type" => "network"));
}

#[derive(ArgEnum, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Protocol {
    /// Transmission Control Protocol
    #[serde(alias = "tcp")]
    TCP,
    /// User Datagram Protocol
    #[serde(alias = "udp")]
    UDP,
}

/// Protocol defaults to UDP
impl Default for Protocol {
    fn default() -> Self {
        Protocol::UDP
    }
}

struct SendResult {
    source: String,
    size: usize,
}

/// Send some data over a protocol to a given destination
pub fn send(dest: &SocketAddr, data: &str, proto: &Protocol) {
    let bytes: &[u8] = &*binary_decode(data);
    let logger = LOGGER.new(o!(
        "cmd" => "send",
        "proto" => format!("{:?}", proto),
        "dest" => dest.to_string(),
    ));
    match match &proto {
        Protocol::TCP => tcp_send(dest, bytes),
        Protocol::UDP => udp_send(dest, bytes),
    } {
        Ok(info) => info!(logger, "ok"; o!("source" => info.source, "size" => info.size )),
        Err(error) => error!(logger, "{}", error),
    }
}

fn tcp_send(destination: &SocketAddr, bytes: &[u8]) -> Result<SendResult, Box<dyn Error>> {
    let mut stream = TcpStream::connect(destination)?;
    Ok(SendResult {
        source: stream.local_addr()?.to_string(),
        size: stream.write(bytes)?,
    })
}

fn udp_send(destination: &SocketAddr, bytes: &[u8]) -> Result<SendResult, Box<dyn Error>> {
    let socket = UdpSocket::bind(match destination.is_ipv6() {
        true => "[::]:0",
        false => "0.0.0.0:0",
    })?;
    Ok(SendResult {
        source: socket.local_addr()?.to_string(),
        size: socket.send_to(bytes, destination)?,
    })
}
