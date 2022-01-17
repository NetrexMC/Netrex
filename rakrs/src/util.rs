use crate::MAGIC;
use binary_utils::error::BinaryError;
use binary_utils::*;
use std::net::{SocketAddr, ToSocketAddrs};

// Raknet utilities
pub trait IPacketStreamWrite {
    fn write_magic(&mut self);

    fn write_address(&mut self, add: SocketAddr);
}

pub trait IPacketStreamRead {
    fn read_magic(&mut self) -> Vec<u8>;

    fn read_address(&mut self) -> SocketAddr;
}

#[derive(Debug)]
pub struct Magic(pub Vec<u8>);

impl Magic {
    pub fn new() -> Self {
        Self(MAGIC.to_vec())
    }
}

impl Streamable for Magic {
    fn parse(&self) -> Result<Vec<u8>, BinaryError> {
        Ok(MAGIC.to_vec())
    }

    fn compose(source: &[u8], position: &mut usize) -> Result<Self, BinaryError> {
        // magic is 16 bytes
        let pos = *position + (16 as usize);
        let magic = &source[*position..pos];
        *position += 16;

        if magic.to_vec() != MAGIC.to_vec() {
            Err(BinaryError::RecoverableKnown(
                "Could not construct magic from malformed bytes.".to_string(),
            ))
        } else {
            Ok(Self(magic.to_vec()))
        }
    }
}

pub fn tokenize_addr(remote: SocketAddr) -> String {
    let mut address = remote.ip().to_string();
    address.push_str(":");
    address.push_str(remote.port().to_string().as_str());
    return address;
}

pub fn from_tokenized(remote: String) -> SocketAddr {
    let mut parsed = remote
        .to_socket_addrs()
        .expect("Could not parse remote address.");
    SocketAddr::from(parsed.next().unwrap())
}
