use binary_utils::{LE, Streamable};
use mcpe_protocol::mcpe::Login;

// handle logins
pub const MOJANG_PUBLIC_KEY: &str = "MHYwEAYHKoZIzj0CAQYFK4EEACIDYgAE8ELkixyLcwlZryUQcu1TvPOmI2B7vX83ndnWRUaXm74wFfa5f/lwQNTfrLVHa2PmenpGI6JhIMUJaWZrjmMj90NoKNFSNBuKdm8rYiXsfaz3K36x/1U26HpG0ZxK/V1V";

#[derive(Debug)]
pub struct LoginData {
	pub protocol: u32,
	pub chainDataJson: String,
	pub clientJson: String,
}

pub fn deconstruct(packet: Login) -> LoginData {
	let protocol = packet.protocol;
	let data = packet.request_data.0; // make this little endian
	let mut pos: usize = 0;
	// todo Read jsonwebtoken data, wtf mojang :(
	let chain = "".to_string();
	let client = "".to_string();
	LoginData { protocol, chainDataJson: chain, clientJson: client}
}