use binary_utils::io::BinaryReader;
use binary_utils::Streamable;
use byteorder::LittleEndian;
use hmac::{Hmac, Mac};
use jwt::{Header, Token, VerifyWithKey};
use mcpe_protocol::interfaces::VarSlice;
use mcpe_protocol::mcpe::Login;
use serde_json::{Map, Value};
use sha2::Sha384;
use std::io::Cursor;

use super::{LoginHandlerError, PlayerLoginData, ProcessedLogin};
use crate::network::session::handler::HandlerError;

const MOJANG_PUBLIC_KEY: &str = "MHYwEAYHKoZIzj0CAQYFK4EEACIDYgAE8ELkixyLcwlZryUQcu1TvPOmI2B7vX83ndnWRUaXm74wFfa5f/lwQNTfrLVHa2PmenpGI6JhIMUJaWZrjmMj90NoKNFSNBuKdm8rYiXsfaz3K36x/1U26HpG0ZxK/V1V";

pub fn decode(packet: Login) -> Result<ProcessedLogin, HandlerError> {
    let raw_data = VarSlice::compose(&packet.request_data.0, &mut 0)?;
    // let _length = VarInt::<u32>::compose(&raw_data, &mut position)?;

    let data = &raw_data.0[..];
    let mut stream = Cursor::new(&data);

    let v = stream.read_string_u32::<LittleEndian>()?;
    let chain: Value = serde_json::from_str(&v)?;
    // this is the client skin data
    let skin_data = stream.read_string_u32::<LittleEndian>()?;
	let mut authorized: bool = false;
	let mut verified: bool = false;

    if let Some(c) = chain.as_object() {
		if c.len() < 1 {
        	return Err(LoginHandlerError::InvalidChain.into());
		}
    } else {
		return Err(LoginHandlerError::InvalidChain.into());
	}

    let chain_data = &chain["chain"];

    // Parse the chain request (0) for the clients token.
    let mut chains: Vec<Map<String, Value>> = Vec::new();

	if chain_data.as_array().is_none() {
		return Err(LoginHandlerError::InvalidChain.into());
	}

	for chain in chain_data.as_array().unwrap() {
		let decoded = decode_chain(chain, &mut authorized, &mut verified)?;
		if let Some(dc) = decoded.as_object() {
			chains.push(dc.clone());
		}
	}

	if let Some(player_data) = chains.get(2) {
		Ok(ProcessedLogin {
			data: get_player_data(player_data["extraData"].clone())?,
			skin_data,
			authorized,
			verified,
		})
	} else {
		Err(LoginHandlerError::InvalidChain.into())
	}
}

/// Decodes the chain and validates the token.
pub fn decode_chain(chain: &Value, auth: &mut bool, verified: &mut bool) -> Result<Value, LoginHandlerError> {
	let token: Token<Header, Value, _> = Token::parse_unverified(chain.as_str().unwrap()).map_err(|_| LoginHandlerError::InvalidChain)?;

	// validate the header
	// this is a bit hacky because our JWT library doesn't allow us to identify the x5u header
	// so we have to parse the header ourselves.
	let header: Value = serde_json::from_slice(&base64::decode(chain.as_str().unwrap().split(".").collect::<Vec<&str>>()[0]).unwrap()).unwrap();

	if header["x5u"].to_string() == MOJANG_PUBLIC_KEY {
		*auth = true;
	} else {
		*auth = false;
	}

	let key: Hmac<Sha384> = Hmac::new_from_slice(MOJANG_PUBLIC_KEY.as_bytes()).unwrap();
	let claims = token.claims().clone();

	if let Ok(_) = token.verify_with_key(&key) {
		*verified = true;
	} else {
		*verified = false;
	}

	return Ok(claims);
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