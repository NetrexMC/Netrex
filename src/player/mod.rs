use std::sync::Arc;

use async_trait::async_trait;
use binary_utils::Streamable;
use mcpe_protocol::interfaces::VarString;
use mcpe_protocol::mcpe::{Disconnect, Packet};

use crate::network::{
    session::Session,
    session::{
        handler::{login::LoginHandler, CanHandle, HandlerError, PlayerHandler},
        SessionInterface,
    },
};

pub mod skin;

// use session::PlayerSession;

/// Generating player data.
pub struct PlayerData {}

impl PlayerData {
    pub fn unknown() -> Self {
        Self {}
    }
}
pub struct Player {
    pub(crate) session: Session,
    pub(crate) name: String,
    pub(crate) display_name: String,
}

impl Player {
    pub fn new(session: Session, data: PlayerData) -> Self {
        Player {
            session,
            name: "".to_string(),
            display_name: "".to_string(),
        }
    }

    pub async fn handle_raw(&mut self, packet: Vec<u8>) -> Result<(), HandlerError> {
        Session::handle_raw(self, packet).await?;
        Ok(())
    }

    pub async fn disconnect<S: Into<String>>(&mut self, reason: S) {
		let reason: String = reason.into();
		self.send(Disconnect {
			hide_screen: false,
			message: VarString(reason.clone()),
		}, true).await;

        self.session.disconnect(reason).await;
    }

    pub async fn send<P: Into<Packet>>(&mut self, packet: P, immediate: bool) {
        self.session.send(packet.into(), immediate).await;
    }

    pub async fn tick(&mut self) {
        self.session.tick().await;
    }
}

#[async_trait]
impl SessionInterface for Player {
    async fn on_packet(&mut self, packet: Packet) -> Result<(), HandlerError> {
        if LoginHandler::can_handle(packet.clone()) {
            let res = LoginHandler::handle(self, packet.clone()).await?;
            if res {
                return Ok(());
            }
        }
        return Err(HandlerError::UnhandledPacket(packet.kind.into()));
    }
}

unsafe impl Send for Player {}
unsafe impl Sync for Player {}
