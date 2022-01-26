use crate::log;
use std::error::Error;
use std::fmt;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::str::FromStr;

lazy_static! {
    static ref LOGGER: slog::Logger = log::LOGGER.new(o!("type" => "network"));
}

#[derive(Debug)]
pub enum Protocol {
    TCP,
    UDP,
}

impl FromStr for Protocol {
    type Err = ProtocolParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_lowercase() {
            "tcp" => Ok(Protocol::TCP),
            "udp" => Ok(Protocol::UDP),
            _ => Err(ProtocolParseError),
        }
    }
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct ProtocolParseError;

impl fmt::Display for ProtocolParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Protocol should be TCP or UDP")
    }
}

impl Error for ProtocolParseError {}

pub fn connect(protocol: &Protocol, destination: &SocketAddr, bytes: &[u8]) {
    let logger = LOGGER.new(o!(
        "cmd" => "connect",
        "protocol" => protocol.to_string(),
        "destination" => destination.to_string(),
    ));
    match match &protocol {
        Protocol::TCP => tcp_send(destination, bytes),
        Protocol::UDP => udp_send(destination, bytes),
    } {
        Ok(info) => info!(logger, "ok"; o!("source" => info.source, "size" => info.size )),
        Err(error) => error!(logger, "{}", error),
    }
}

struct SendInfo {
    source: String,
    size: usize,
}

fn tcp_send(destination: &SocketAddr, bytes: &[u8]) -> Result<SendInfo, Box<dyn Error>> {
    let mut stream = TcpStream::connect(destination)?;
    Ok(SendInfo {
        source: stream.local_addr()?.to_string(),
        size: stream.write(bytes)?,
    })
}

fn udp_send(destination: &SocketAddr, bytes: &[u8]) -> Result<SendInfo, Box<dyn Error>> {
    let socket = UdpSocket::bind(match destination.is_ipv6() {
        true => "[::]:0",
        false => "0.0.0.0:0",
    })?;
    Ok(SendInfo {
        source: socket.local_addr()?.to_string(),
        size: socket.send_to(bytes, destination)?,
    })
}
