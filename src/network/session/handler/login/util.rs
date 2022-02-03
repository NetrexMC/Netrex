use binary_utils::io::BinaryReader;
use binary_utils::{Streamable, VarInt, LE};
use byteorder::LittleEndian;
use jwt::{AlgorithmType, Header, Token, VerifyWithKey};
use mcpe_protocol::interfaces::{LString32, String32, VarSlice};
use mcpe_protocol::mcpe::Login;
use serde_json::{Map, Value};
use std::fs::{File, OpenOptions};
use std::io::{Cursor, Write};

use super::{LoginHandlerError, PlayerLoginData, ProcessedLogin};
use crate::network::session::handler::HandlerError;
use crate::network::session::Session;

const MOJANG_PUBLIC_KEY: &str = "MHYwEAYHKoZIzj0CAQYFK4EEACIDYgAE8ELkixyLcwlZryUQcu1TvPOmI2B7vX83ndnWRUaXm74wFfa5f/lwQNTfrLVHa2PmenpGI6JhIMUJaWZrjmMj90NoKNFSNBuKdm8rYiXsfaz3K36x/1U26HpG0ZxK/V1V";

pub fn decode(packet: Login) -> Result<ProcessedLogin, HandlerError> {
    let protocol = packet.protocol;

    let raw_data = VarSlice::compose(&packet.request_data.0, &mut 0)?;
    // let _length = VarInt::<u32>::compose(&raw_data, &mut position)?;

    let data = &raw_data.0[..];
    let mut stream = Cursor::new(&data);

    let v = stream.read_string_u32::<LittleEndian>()?;
    let chain: Value = serde_json::from_str(&v)?;
    // this is the client skin data
    let skin_data = stream.read_string_u32::<LittleEndian>()?;

    if chain.as_object().unwrap().len() < 1 {
        return Err(LoginHandlerError::InvalidChain.into());
    }

    let chain_data = &chain["chain"];

    let mut f = OpenOptions::new()
        .append(true)
        .read(true)
        .create(true)
        .open("chain.debug")
        .unwrap();

    // Parse the chain request (0) for the clients token.
    for val in chain_data.as_array().unwrap() {
        let c = decode_chain(val, "")?;
        f.write_all(format!("{:?}\n", c).as_bytes()).unwrap();
    }

    Ok(ProcessedLogin {
        data: todo!(),
        authorized: todo!(),
        verified: todo!(),
    })
}

/// Decodes the chain and validates the token.
pub fn decode_chain<S: Into<String>>(chain: &Value, key: S) -> Result<Value, LoginHandlerError> {
    let token: Token<Header, Value, _> = Token::parse_unverified(chain.as_str().unwrap()).unwrap();

    if !token
        .header()
        .key_id
        .as_ref()
        .contains(&&MOJANG_PUBLIC_KEY.to_string())
    {
        return Err(LoginHandlerError::InvalidChain);
    }

    return Ok(token.claims().clone());
}

pub fn get_player_data(chain_data: Value) -> Result<PlayerLoginData, HandlerError> {
    let name = chain_data["name"].to_string();
    let id = chain_data["id"].to_string();
    let os_id = chain_data["os"].to_string();
    let xuid = chain_data["xuid"].to_string();
    Ok(PlayerLoginData {
        name,
        id,
        os_id,
        xuid,
    })
}

// This was taken from PMMP
pub async fn validate_token(token: &str, public_key: &str) -> Result<bool, ()> {
    Ok(true)
}
