pub mod login;

use async_trait::async_trait;
use binary_utils::error::BinaryError;
use mcpe_protocol::mcpe::Packet;
use serde_json::Error as SerdeJsonError;

use crate::player::Player;

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
    PacketDecodeError,
    BinaryError(BinaryError),
    SerdeJsonError(SerdeJsonError),
}

impl_err_handler!(String, UnhandledPacket);
impl_err_handler!(BinaryError, BinaryError);
impl_err_handler!(SerdeJsonError, SerdeJsonError);

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
