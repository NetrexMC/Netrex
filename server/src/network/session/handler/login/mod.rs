mod util;
use util::*;

use async_trait::async_trait;
use mcpe_protocol::mcpe::{ClientToServerHandshake, Login, Packet, PacketId, version_within_current_minor, PlayStatus, CURRENT_MAJOR};

use crate::player::Player;

use super::{CanHandle, PlayerHandler};

#[derive(Debug, Clone)]
pub enum LoginHandlerError {
    Known(String),
    InvalidChain,
    InvalidClientData,
	BrokenSignature,
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
	/// The skin data of the player
	/// At this stage, this is Base64 encoded
	pub skin_data: String,
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
		let login_packet: Login = packet.kind.clone().into();
        let login = decode(packet.kind.into())?;

		if login.verified != true {
			return Err(LoginHandlerError::BrokenSignature.into());
		}

		// this second true represent whether or not auth is enabled.
		// this is a to-do
		if login.authorized != true && true {
			player.disconnect("You must be logged in with your Xbox account to play on this server.").await;
			return Ok(false);
		}

		if version_within_current_minor(login_packet.protocol) {
        	return Ok(true);
		} else if login_packet.protocol < CURRENT_MAJOR {
			player.send(PlayStatus::FailedClient, true);
			player.disconnect("Outdated client");
			return Ok(false);
		} else {
			player.send(PlayStatus::FailedServer, true);
			player.disconnect("Outdated server");
			return Ok(false);
		}
    }
}

impl CanHandle for LoginHandler {
    fn can_handle(packet: Packet) -> bool {
        packet.id == Login::id() || packet.id == ClientToServerHandshake::id()
    }
}
