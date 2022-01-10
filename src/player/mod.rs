use binary_utils::Streamable;
use mcpe_protocol::mcpe::Packet;

use crate::network::{
    handler::{login::LoginHandler, CanHandle, HandlerError, PlayerHandler},
    session::Session,
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

    pub async fn handle(&mut self, buffer: Vec<u8>) -> Result<(), HandlerError> {
        let maybe_pk = Packet::compose(&buffer[..], &mut 0);

        if let Ok(packet) = maybe_pk {
            if LoginHandler::can_handle(packet.clone()) {
                let res = LoginHandler::handle(self, packet.clone()).await?;
                if res {
                    return Ok(());
                }
            }
            return Err(HandlerError::UnhandledPacket(packet.kind.into()));
        } else {
            return Err(HandlerError::PacketDecodeError);
        }
    }
}

unsafe impl Send for Player {}
unsafe impl Sync for Player {}
