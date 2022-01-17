#![allow(dead_code)]
use crate::conn::{Connection, ConnectionState};
use crate::util::tokenize_addr;
use crate::RakEvent;
use binary_utils::error::BinaryError;
use binary_utils::*;
use byteorder::{BigEndian, WriteBytesExt};
use std::fmt::{Formatter, Result as FResult};
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq)]
pub enum OnlinePackets {
    ConnectedPing,
    ConnectedPong,
    ConnectionRequest,
    ConnectionAccept,
    GamePacket,
    FramePacket(u8),
    NewConnection,
    Disconnect,
    UnknownPacket(u8),
}

impl OnlinePackets {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0x00 => OnlinePackets::ConnectedPing,
            0x03 => OnlinePackets::ConnectedPong,
            0x09 => OnlinePackets::ConnectionRequest,
            0x10 => OnlinePackets::ConnectionAccept,
            0x13 => OnlinePackets::NewConnection,
            0x15 => OnlinePackets::Disconnect,
            0xfe => OnlinePackets::GamePacket,
            0x80..=0x8d => OnlinePackets::FramePacket(byte),
            _ => OnlinePackets::UnknownPacket(byte),
        }
    }

    pub fn to_byte(&self) -> u8 {
        match *self {
            OnlinePackets::ConnectedPing => 0x00,
            OnlinePackets::ConnectedPong => 0x03,
            OnlinePackets::ConnectionRequest => 0x09,
            OnlinePackets::ConnectionAccept => 0x10,
            OnlinePackets::NewConnection => 0x13,
            OnlinePackets::Disconnect => 0x15,
            OnlinePackets::GamePacket => 0xfe,
            OnlinePackets::FramePacket(b) => b,
            OnlinePackets::UnknownPacket(byte) => byte,
        }
    }

    pub fn is_unknown(&self) -> bool {
        match *self {
            OnlinePackets::UnknownPacket(_) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for OnlinePackets {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match *self {
            OnlinePackets::ConnectedPing => write!(f, "ConnectedPing()"),
            OnlinePackets::ConnectedPong => write!(f, "ConnectedPong()"),
            OnlinePackets::ConnectionRequest => write!(f, "ConnectionRequest()"),
            OnlinePackets::ConnectionAccept => write!(f, "ConnectionAccept()"),
            OnlinePackets::NewConnection => write!(f, "NewConnection()"),
            OnlinePackets::Disconnect => write!(f, "Disconnect()"),
            OnlinePackets::GamePacket => write!(f, "GamePacket()"),
            OnlinePackets::UnknownPacket(byte) => write!(f, "UnknownPacket(ID={:#04x})", byte),
            OnlinePackets::FramePacket(byte) => write!(f, "FramePacket(ID={:#04x})", byte),
        }
    }
}

#[derive(Debug, BinaryStream)]
pub struct ConnectionRequest {
    client_id: i64,
    timestamp: i64,
}

#[derive(Debug)]
pub struct ConnectionAccept {
    id: u8,
    client_address: SocketAddr,
    system_index: i16,
    internal_ids: SocketAddr,
    request_time: i64,
    timestamp: i64,
}

impl Streamable for ConnectionAccept {
    fn parse(&self) -> Result<Vec<u8>, BinaryError> {
        let mut stream = Vec::new();
        stream.write_u8(self.id)?;
        stream.write_all(&self.client_address.parse()?[..])?;
        stream.write_i16::<BigEndian>(self.system_index)?;
        for _ in 0..10 {
            stream.write_all(&self.internal_ids.parse()?[..])?;
        }
        stream.write_i64::<BigEndian>(self.request_time)?;
        stream.write_i64::<BigEndian>(self.timestamp)?;
        Ok(stream)
    }

    fn compose(_source: &[u8], _position: &mut usize) -> Result<Self, BinaryError> {
        Ok(Self {
            id: 0,
            client_address: SocketAddr::new(IpAddr::from(Ipv4Addr::new(192, 168, 0, 1)), 9120),
            system_index: 0,
            internal_ids: SocketAddr::new(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), 1920),
            request_time: 0,
            timestamp: 0,
        })
    }
}

#[derive(Debug, BinaryStream)]
pub struct ConnectedPing {
    time: i64,
}

#[derive(Debug, BinaryStream)]
pub struct ConnectedPong {
    id: u8,
    ping_time: i64,
    pong_time: i64,
}

pub fn log_online(message: String) {
    if cfg!(any(test, feature = "dbg")) {
        println!("[RakNet] [Online Packet Handler] {}", message);
    }
}

pub fn handle_online(
    connection: &mut Connection,
    pk: OnlinePackets,
    stream: &mut Vec<u8>,
) -> Result<Vec<u8>, BinaryError> {
    log_online(format!(
        "[{}] Received packet: {}",
        &connection.address, &pk
    ));
    match pk {
        OnlinePackets::ConnectionRequest => {
            let request = ConnectionRequest::compose(stream, &mut 1)?;
            let accept = ConnectionAccept {
                id: OnlinePackets::ConnectionAccept.to_byte(),
                client_address: connection.address.clone(),
                system_index: 0,
                internal_ids: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255)), 19132),
                request_time: request.timestamp,
                timestamp: SystemTime::now()
                    .duration_since(connection.time)
                    .unwrap()
                    .as_millis() as i64,
            };
            connection.state = ConnectionState::Connected;
            accept.parse()
        }
        OnlinePackets::Disconnect => {
            connection.state = ConnectionState::Offline;
            connection.event_dispatch.push_back(RakEvent::Disconnect(
                tokenize_addr(connection.address),
                "Client disconnect".to_owned(),
            ));
            Ok(Vec::new())
        }
        OnlinePackets::NewConnection => Ok(Vec::new()),
        OnlinePackets::ConnectedPing => {
            let request = ConnectedPing::compose(stream, &mut 0)?;
            let pong = ConnectedPong {
                id: OnlinePackets::ConnectedPong.to_byte(),
                ping_time: request.time,
                pong_time: SystemTime::now()
                    .duration_since(connection.time)
                    .unwrap()
                    .as_millis() as i64,
            };
            pong.parse()
        }
        OnlinePackets::FramePacket(_v) => {
            // println!("Condition should never be met.");
            Ok(Vec::new())
        }
        _ => Ok(Vec::new()), // TODO: Throw an UnknownPacket here rather than sending an empty binary stream
    }
}
