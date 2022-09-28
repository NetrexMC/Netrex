pub mod login;
pub mod raw;

use std::io::Error as IoError;

use async_trait::async_trait;
use binary_utils::error::BinaryError;
use mcpe_protocol::mcpe::Packet;
use serde_json::Error as SerdeJsonError;

use crate::player::Player;

use self::{login::LoginHandlerError, raw::RawHandlerError};

macro_rules! impl_err_handler {
    ($name: ident, $mtd: ident) => {
        impl From<$name> for HandlerError {
            fn from(err: $name) -> HandlerError {
                HandlerError::$mtd(err)
            }
        }
    };
}

#[derive(Debug)]
pub enum HandlerError {
    UnhandledPacket(String),
    UnknownError(String),
    PacketDecodeError,
    LoginHandlerError(LoginHandlerError),
    RawHandlerError(RawHandlerError),
    BinaryError(BinaryError),
    SerdeJsonError(SerdeJsonError),
    IoError(IoError),
}

impl HandlerError {
    pub fn is_login_error(&self) -> bool {
        match self {
            HandlerError::LoginHandlerError(_) => true,
            _ => false,
        }
    }

    pub fn is_raw_error(&self) -> bool {
        match self {
            HandlerError::RawHandlerError(_) => true,
            _ => false,
        }
    }

    pub fn is_binary_error(&self) -> bool {
        match self {
            HandlerError::BinaryError(_) => true,
            _ => false,
        }
    }

    pub fn is_serde_json_error(&self) -> bool {
        match self {
            HandlerError::SerdeJsonError(_) => true,
            _ => false,
        }
    }

    pub fn is_io_error(&self) -> bool {
        match self {
            HandlerError::IoError(_) => true,
            _ => false,
        }
    }

    pub fn is_unknown_error(&self) -> bool {
        match self {
            HandlerError::UnknownError(_) => true,
            _ => false,
        }
    }

    pub fn is_packet_decode_error(&self) -> bool {
        match self {
            HandlerError::PacketDecodeError => true,
            _ => false,
        }
    }

    pub fn is_unhandled_packet(&self) -> bool {
        match self {
            HandlerError::UnhandledPacket(_) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for HandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandlerError::UnhandledPacket(name) => write!(f, "Unhandled Packet: {}", name),
            HandlerError::UnknownError(name) => write!(f, "Unknown Error: {}", name),
            HandlerError::PacketDecodeError => write!(f, "Packet Decode Error"),
            HandlerError::LoginHandlerError(err) => write!(f, "Login Error: {:?}", err),
            HandlerError::RawHandlerError(err) => write!(f, "Raw Error: {:?}", err),
            HandlerError::BinaryError(err) => write!(f, "Binary Error: {}", err),
            HandlerError::SerdeJsonError(err) => write!(f, "Serde json Error: {}", err),
            HandlerError::IoError(err) => write!(f, "Io Error: {}", err),
        }
    }
}
impl_err_handler!(String, UnhandledPacket);
impl_err_handler!(BinaryError, BinaryError);
impl_err_handler!(LoginHandlerError, LoginHandlerError);
impl_err_handler!(RawHandlerError, RawHandlerError);
impl_err_handler!(SerdeJsonError, SerdeJsonError);
impl_err_handler!(IoError, IoError);

/// Handlers for network interfaces.
/// A handler is a function that takes a packet and, well
/// handles it.
#[async_trait]
pub trait PlayerHandler {
    /// Handle a packet.
    /// This is called when a packet is received.
    /// The handler should return `Ok(false)` if the packet can not be handled.
    /// Otherwise, it should return `Ok(true)` to acknowledge the packet was handled.
    async fn handle(parent: &mut Player, packet: Packet) -> Result<bool, HandlerError>;
}

pub trait CanHandle {
    /// Check if the handler can handle the packet.
    /// This is called before the handler is called.
    /// The handler should return `true` if the handler can handle the packet.
    fn can_handle(packet: Packet) -> bool;
}
