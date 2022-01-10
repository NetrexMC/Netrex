use async_trait::async_trait;
use binary_utils::{Streamable, LE};
use mcpe_protocol::interfaces::{LString32, VarSlice};
use mcpe_protocol::mcpe::{Login, Packet, PacketId};
use serde_json::Value;

use crate::network::session::Session;
use crate::player::Player;

use super::{CanHandle, HandlerError, PlayerHandler};
const MOJANG_PUBLIC_KEY: &str = "MHYwEAYHKoZIzj0CAQYFK4EEACIDYgAE8ELkixyLcwlZryUQcu1TvPOmI2B7vX83ndnWRUaXm74wFfa5f/lwQNTfrLVHa2PmenpGI6JhIMUJaWZrjmMj90NoKNFSNBuKdm8rYiXsfaz3K36x/1U26HpG0ZxK/V1V";

#[derive(Clone, Debug)]
pub struct LoginData {
    pub protocol: u32,
    pub chain_data: Value,
    pub client_data: String,
}

#[derive(Clone, Debug)]
pub struct LoginHandler;

impl LoginHandler {
    pub fn decode(packet: Login) -> Result<LoginData, HandlerError> {
        let protocol = packet.protocol;
        let data = packet.request_data.0; // make this little endian
        let chain_raw = VarSlice::fcompose(&data[..], &mut 0);
        let mut pos: usize = 0;
        let chain_data = serde_json::from_str(&LString32::fcompose(&chain_raw.0[..], &mut pos).0)?;
        let client_data = LString32::fcompose(&chain_raw.0[..], &mut pos).0;
        Ok(LoginData {
            protocol,
            chain_data,
            client_data,
        })
    }
}

#[async_trait]
impl PlayerHandler for LoginHandler {
    async fn handle(
        player: &mut Player,
        packet: mcpe_protocol::mcpe::Packet,
    ) -> Result<bool, super::HandlerError> {
        let login_data = Self::decode(packet.kind.into())?;
        // println!("{:?}", login_data);
        return Ok(false);
    }
}

impl CanHandle for LoginHandler {
    fn can_handle(packet: Packet) -> bool {
        packet.id == Login::id()
    }
}
