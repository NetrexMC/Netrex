mod util;
use util::*;

use async_trait::async_trait;
use mcpe_protocol::mcpe::{ClientToServerHandshake, Login, Packet, PacketId};
use serde_json::Value;

use crate::player::Player;

use super::{CanHandle, PlayerHandler};

#[derive(Debug, Clone)]
pub enum LoginHandlerError {
    Known(String),
    InvalidChain,
    InvalidClientData,
    ProcessError(ProcessedLogin),
}

#[derive(Clone, Debug)]
pub struct PlayerLoginData {
    /// The Name of the player
    pub name: String,
    /// The UUID of the player
    pub id: String,
    /// The device the player is playing on
    pub os_id: String,
    /// The xbox user id
    pub xuid: String,
}

impl PlayerLoginData {
    pub fn new(name: String, id: String, os_id: String, xuid: String) -> Self {
        Self {
            name,
            id,
            os_id,
            xuid,
        }
    }

    pub fn empty() -> Self {
        Self {
            name: String::new(),
            id: String::new(),
            os_id: String::new(),
            xuid: String::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProcessedLogin {
    /// The data relating to the player attempting to login
    pub data: PlayerLoginData,
    /// Whether or not the login was signed by mojang (XBL)
    pub authorized: bool,
    /// Whether or not the signature is broken
    /// (i.e. the player data has been tampered with)
    pub verified: bool,
}

#[derive(Clone, Debug)]
pub struct LoginHandler;

#[async_trait]
impl PlayerHandler for LoginHandler {
    async fn handle(
        player: &mut Player,
        packet: mcpe_protocol::mcpe::Packet,
    ) -> Result<bool, super::HandlerError> {
        let login = decode(packet.kind.into())?;

        return Ok(true);
    }
}

impl CanHandle for LoginHandler {
    fn can_handle(packet: Packet) -> bool {
        packet.id == Login::id() || packet.id == ClientToServerHandshake::id()
    }
}
