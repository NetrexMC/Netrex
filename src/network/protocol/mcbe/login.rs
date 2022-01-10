use base64;
use binary_utils::{Streamable, LE};
use mcpe_protocol::interfaces::{LString32, VarSlice};
use mcpe_protocol::mcpe::Login;
use serde_json::{Result as SResult, Value};

use crate::server::Server;

// handle logins
pub const MOJANG_PUBLIC_KEY: &str = "MHYwEAYHKoZIzj0CAQYFK4EEACIDYgAE8ELkixyLcwlZryUQcu1TvPOmI2B7vX83ndnWRUaXm74wFfa5f/lwQNTfrLVHa2PmenpGI6JhIMUJaWZrjmMj90NoKNFSNBuKdm8rYiXsfaz3K36x/1U26HpG0ZxK/V1V";

#[derive(Debug)]
pub struct LoginData {
    pub protocol: u32,
    pub chain_data: Value,
    pub client_data: String,
}

pub fn decode_login(packet: Login) -> Result<LoginData, serde_json::Error> {
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

pub fn do_login(server: &mut Server, address: String, packet: Login) {
    if let Ok(login_data) = decode_login(packet) {
        // send play status
        if login_data.protocol > 400 {
        } else {
        }
    } else {
    }
}
