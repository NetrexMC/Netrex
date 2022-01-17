#![allow(dead_code)]

use crate::conn::{Connection, ConnectionState};
use crate::{Magic, RakEvent, RakNetVersion, USE_SECURITY};
use binary_utils::error::BinaryError;
use binary_utils::*;
use byteorder::WriteBytesExt;
use std::fmt::{Formatter, Result as FResult};
use std::io::Write;
use std::net::SocketAddr;

pub enum OfflinePackets {
    UnconnectedPing,
    OpenConnectRequest,
    OpenConnectReply,
    SessionInfoRequest,
    SessionInfoReply,
    UnconnectedPong,
    IncompatibleProtocolVersion,
    UnknownPacket(u8),
}

impl OfflinePackets {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0x01 => OfflinePackets::UnconnectedPing,
            0x05 => OfflinePackets::OpenConnectRequest,
            0x06 => OfflinePackets::OpenConnectReply,
            0x07 => OfflinePackets::SessionInfoRequest,
            0x08 => OfflinePackets::SessionInfoReply,
            0x1c => OfflinePackets::UnconnectedPong,
            0x19 => OfflinePackets::IncompatibleProtocolVersion,
            _ => OfflinePackets::UnknownPacket(byte),
        }
    }

    pub fn to_byte(&self) -> u8 {
        match *self {
            OfflinePackets::UnconnectedPing => 0x01,
            OfflinePackets::OpenConnectRequest => 0x05,
            OfflinePackets::OpenConnectReply => 0x06,
            OfflinePackets::SessionInfoRequest => 0x07,
            OfflinePackets::SessionInfoReply => 0x08,
            OfflinePackets::UnconnectedPong => 0x1c,
            OfflinePackets::IncompatibleProtocolVersion => 0x19,
            OfflinePackets::UnknownPacket(byte) => byte,
        }
    }
}

impl std::fmt::Display for OfflinePackets {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match *self {
            OfflinePackets::UnconnectedPing => write!(f, "UnconnectedPing()"),
            OfflinePackets::OpenConnectRequest => write!(f, "OpenConnectRequest()"),
            OfflinePackets::OpenConnectReply => write!(f, "OpenConnectReply()"),
            OfflinePackets::SessionInfoRequest => write!(f, "SessionInfoRequest()"),
            OfflinePackets::SessionInfoReply => write!(f, "SessionInfoReply()"),
            OfflinePackets::UnconnectedPong => write!(f, "UnconnectedPong()"),
            OfflinePackets::IncompatibleProtocolVersion => {
                write!(f, "IncompatibleProtocolVersion()")
            }
            OfflinePackets::UnknownPacket(byte) => write!(f, "UnknownPacket(ID={:#04x})", byte),
        }
    }
}

/// Unconnected Ping
#[derive(Debug, BinaryStream)]
pub struct UnconnectedPing {
    timestamp: u64,
    magic: Magic,
    client_id: i64,
}

/// Unconnected Pong
#[derive(Debug, BinaryStream)]
pub struct UnconnectedPong {
    id: u8,
    timestamp: u64,
    server_id: u64,
    magic: Magic,
    motd: String,
}

/// A connection request recv the client.
#[derive(Debug)]
pub struct OpenConnectRequest {
    magic: Magic,
    protocol: u8,
    mtu_size: u16,
}

impl Streamable for OpenConnectRequest {
    fn compose(source: &[u8], position: &mut usize) -> Result<Self, BinaryError> {
        Ok(Self {
            magic: Magic::compose(source, position)?,
            protocol: u8::compose(source, position)?,
            mtu_size: (source.len() + 28) as u16,
        })
    }

    fn parse(&self) -> Result<Vec<u8>, BinaryError> {
        let mut stream = Vec::<u8>::new();
        stream
            .write(&self.magic.parse()?[..])
            .expect("Failed to parse open connect request");
        stream.write_u8(self.protocol)?;
        // padding
        for _ in 0..self.mtu_size {
            stream.write_u8(0)?;
        }
        Ok(stream)
    }
}

// Mtu size may be needed here.
// impl IServerBound<OpenConnectRequest> for OpenConnectRequest {
//      fn recv(mut s: Stream) -> OpenConnectRequest {
//           let magic = s.read_magic();
//           let p = s.read_byte();
//           let mtu = s.get_length() + 1 + 28;
//           Self {
//                magic,
//                protocol: p,
//                mtu_size: mtu as i16,
//           }
//      }
// }

/// Open Connection Reply
/// Sent to the client when the server accepts a client.
#[derive(Debug, BinaryStream)]
pub struct OpenConnectReply {
    id: u8,
    magic: Magic,
    server_id: u64,
    security: bool,
    mtu_size: u16,
}
/// Session info, also known as Open Connect Request 2
#[derive(Debug, BinaryStream)]
pub struct SessionInfoRequest {
    magic: Magic,
    address: SocketAddr,
    mtu_size: u16,
    client_id: i64,
}

/// Session Info Reply, also known as Open Connect Reply 2
#[derive(Debug, BinaryStream)]
pub struct SessionInfoReply {
    id: u8,
    magic: Magic,
    server_id: u64,
    client_address: SocketAddr,
    mtu_size: u16,
    security: bool,
}

#[derive(Debug, BinaryStream)]
pub struct IncompatibleProtocolVersion {
    id: u8,
    protocol: u8,
    magic: Magic,
    server_id: u64,
}

pub fn log_offline(message: String) {
    if cfg!(any(test, feature = "dbg")) {
        println!("[RakNet] [Offline Packet Handler] {}", message);
    }
}

pub fn handle_offline(
    connection: &mut Connection,
    pk: OfflinePackets,
    stream: &mut &Vec<u8>,
) -> Result<Vec<u8>, BinaryError> {
    log_offline(format!(
        "[{}] Received packet: {}",
        &connection.address, &pk
    ));
    match pk {
        OfflinePackets::UnconnectedPing => {
            connection.event_dispatch.push_back(RakEvent::Motd(
                connection.address_token.clone(),
                connection.motd.clone(),
            ));

            let pong = UnconnectedPong {
                id: OfflinePackets::UnconnectedPong.to_byte(),
                server_id: connection.server_guid,
                timestamp: connection.time.elapsed().unwrap().as_millis() as u64,
                magic: Magic::new(),
                motd: connection.get_motd().encode(),
            };

            // // println!("Pong MOTD: {:?}", pong.motd.parse()?);
            // // println!("[RakNet] [{}] Pong data: {:?}", &connection.address, pong.motd);
            pong.parse()
        }
        OfflinePackets::OpenConnectRequest => {
            let request = OpenConnectRequest::compose(&stream[..], &mut 1)?;

            if request.protocol != RakNetVersion::MinecraftRecent.to_u8() {
                let incompatible = IncompatibleProtocolVersion {
                    id: OfflinePackets::IncompatibleProtocolVersion.to_byte(),
                    protocol: request.protocol,
                    magic: Magic::new(),
                    server_id: connection.server_guid,
                };

                return incompatible.parse();
            }

            let reply = OpenConnectReply {
                id: OfflinePackets::OpenConnectReply.to_byte(),
                server_id: connection.server_guid,
                security: USE_SECURITY,
                magic: Magic::new(),
                mtu_size: request.mtu_size,
            };

            reply.parse()
        }
        OfflinePackets::SessionInfoRequest => {
            let request = SessionInfoRequest::compose(&stream[..], &mut 1)?;
            let reply = SessionInfoReply {
                id: OfflinePackets::SessionInfoReply.to_byte(),
                server_id: connection.server_guid,
                client_address: connection.address.clone(),
                magic: Magic::new(),
                mtu_size: request.mtu_size,
                security: USE_SECURITY,
            };

            connection.mtu_size = request.mtu_size as u16;
            connection.state = ConnectionState::Connecting;
            reply.parse()
        }
        _ => Ok(Vec::new()), //TODO: Throw an UnknownPacket here rather than sending an empty binary stream
    }
}
