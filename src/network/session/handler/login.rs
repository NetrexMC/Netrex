use std::fs::{File, OpenOptions};
use std::io::{Cursor, Write};

use async_trait::async_trait;
use binary_utils::io::BinaryReader;
use binary_utils::{Streamable, VarInt, LE};
use byteorder::LittleEndian;
use jwt::{AlgorithmType, Header, Token, VerifyWithKey};
use mcpe_protocol::interfaces::{LString32, String32, VarSlice};
use mcpe_protocol::mcpe::{Login, Packet, PacketId, PlayStatus};
use serde_json::{Map, Value};

use crate::network::session::Session;
use crate::player::Player;

use super::{CanHandle, HandlerError, PlayerHandler};
const MOJANG_PUBLIC_KEY: &str = "MHYwEAYHKoZIzj0CAQYFK4EEACIDYgAE8ELkixyLcwlZryUQcu1TvPOmI2B7vX83ndnWRUaXm74wFfa5f/lwQNTfrLVHa2PmenpGI6JhIMUJaWZrjmMj90NoKNFSNBuKdm8rYiXsfaz3K36x/1U26HpG0ZxK/V1V";

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

impl LoginHandler {
    pub fn decode(packet: Login) -> Result<PreLoginData, HandlerError> {
        let protocol = packet.protocol;
        let raw_data = packet.request_data.0;

        let mut position: usize = 0;
        let length = VarInt::<u32>::compose(&raw_data, &mut position)?;

        let data = &raw_data[..];
        let mut stream = Cursor::new(&data);

        let v = stream.read_string_u32::<LittleEndian>()?;

        let client_data = stream.read_string_u32::<LittleEndian>()?;
        let chain_data = serde_json::from_str(&v)?;

        // read LE lengthed string.

        return Ok(PreLoginData {
            protocol,
            chain_data,
            client_data,
        });
    }

    pub fn decode_prelogin(prelogin: PreLoginData) -> Result<String, HandlerError> {
        // do initial chain data check
        if prelogin.chain_data.as_object().unwrap().len() < 1 {
            return Err(LoginHandlerError::Known("Invalid chain data".to_string()).into());
        }

        let chain_data = prelogin
            .chain_data
            .as_object()
            .unwrap()
            .get("chain")
            .unwrap()
            .as_array()
            .unwrap();
        println!("{:?}", chain_data);

        let mut f = OpenOptions::new()
            .append(true)
            .read(true)
            .create(true)
            .open("chain.debug")
            .unwrap();
        // Parse the chain request (0) for the clients token.
        for val in chain_data {
            let mut chain = Self::decode_chain(val, "")?;
            f.write_all(format!("{:?}\n", chain).as_bytes()).unwrap();
        }
        Ok("t".into())
    }

    fn decode_chain<S: Into<String>>(chain: &Value, key: S) -> Result<Value, LoginHandlerError> {
        let token: Token<Header, Value, _> =
            Token::parse_unverified(chain.as_str().unwrap()).unwrap();

        // Verify the token
        return Ok(token.claims().clone());
    }

    fn validate_chain(&self, chain_data: Value) -> Result<LoginChainData, HandlerError> {
        let obj = chain_data.as_object().unwrap();
        let name = obj.get("name").unwrap().as_str().unwrap().to_string();
        let id = obj.get("id").unwrap().as_str().unwrap().to_string();
        let os_id = obj.get("os").unwrap().as_str().unwrap().to_string();
        let xuid = obj.get("xuid").unwrap().as_str().unwrap().to_string();
        Ok(LoginChainData {
            name,
            id,
            os_id,
            xuid,
        })
    }
}

#[async_trait]
impl PlayerHandler for LoginHandler {
    async fn handle(
        player: &mut Player,
        packet: mcpe_protocol::mcpe::Packet,
    ) -> Result<bool, super::HandlerError> {
        player.send(PlayStatus::Success.into(), true).await;
        // fuck this shit
        // write the stream manually
        // let buf: &[u8] = &[254, 0, 6, 0, 249, 255, 5, 2, 0, 0, 0, 1, 3, 0];
        // player.session.send_stream(buf.to_vec()).await;
        let login_data = Self::decode(packet.kind.into())?;
        Self::decode_prelogin(login_data)?;
        return Ok(false);
    }
}

impl CanHandle for LoginHandler {
    fn can_handle(packet: Packet) -> bool {
        packet.id == Login::id()
    }
}
