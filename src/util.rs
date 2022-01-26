use std::str::FromStr;
use substring::Substring;

pub struct BinaryData {
    pub bytes: Vec<u8>,
}

impl FromStr for BinaryData {
    type Err = hex::FromHexError;
    fn from_str(data: &str) -> Result<Self, Self::Err> {
        match data.substring(0, 2) {
            "0x" => match hex::decode(data[2..].as_bytes()) {
                Ok(bytes) => Ok(BinaryData { bytes }),
                Err(error) => Err(error),
            },
            _ => Ok(BinaryData {
                bytes: data.as_bytes().to_vec(),
            }),
        }
    }
}
