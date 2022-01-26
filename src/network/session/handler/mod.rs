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

impl std::fmt::Display for HandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandlerError::UnhandledPacket(name) => write!(f, "Unhandled packet: {}", name),
            HandlerError::UnknownError(name) => write!(f, "Unknown error: {}", name),
            HandlerError::PacketDecodeError => write!(f, "Packet decode error"),
            HandlerError::LoginHandlerError(err) => write!(f, "Login handler error: {:?}", err),
            HandlerError::RawHandlerError(err) => write!(f, "Raw handler error: {:?}", err),
            HandlerError::BinaryError(err) => write!(f, "Binary error: {}", err),
            HandlerError::SerdeJsonError(err) => write!(f, "Serde json error: {}", err),
            HandlerError::IoError(err) => write!(f, "Io error: {}", err),
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
