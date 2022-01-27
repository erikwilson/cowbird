use serde::{Deserialize, Serialize};
use std::error;
use std::fmt;
use std::str;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(alias = "tcp")]
    TCP,
    #[serde(alias = "udp")]
    UDP,
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::UDP
    }
}

#[derive(Debug)]
pub struct ProtocolParseError;

impl error::Error for ProtocolParseError {}

impl fmt::Display for ProtocolParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Protocol should be TCP or UDP")
    }
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl str::FromStr for Protocol {
    type Err = ProtocolParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_lowercase() {
            "tcp" => Ok(Protocol::TCP),
            "udp" => Ok(Protocol::UDP),
            _ => Err(ProtocolParseError),
        }
    }
}
