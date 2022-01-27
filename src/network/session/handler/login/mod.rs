mod util;
use util::*;

use async_trait::async_trait;
use mcpe_protocol::mcpe::{Login, Packet, PacketId};
use serde_json::Value;

use crate::player::Player;

use super::{CanHandle, PlayerHandler};

#[derive(Debug, Clone)]
pub enum LoginHandlerError {
    Known(String),
}

#[derive(Clone, Debug)]
pub struct PreLoginData {
    pub protocol: u32,
    pub chain_data: Value,
    pub client_data: String,
}

#[derive(Clone, Debug)]
pub struct LoginChainData {
    /// The Name of the player
    pub name: String,
    /// The UUID of the player
    pub id: String,
    /// The device the player is playing on
    pub os_id: String,
    /// The xbox user id
    pub xuid: String,
}

#[derive(Clone, Debug)]
pub struct LoginHandler;

#[async_trait]
impl PlayerHandler for LoginHandler {
    async fn handle(
        player: &mut Player,
        packet: mcpe_protocol::mcpe::Packet,
    ) -> Result<bool, super::HandlerError> {
        let login_data = decode(packet.kind.into())?;
        decode_prelogin(login_data)?;
        return Ok(true);
    }
}

impl CanHandle for LoginHandler {
    fn can_handle(packet: Packet) -> bool {
        packet.id == Login::id()
    }
}
