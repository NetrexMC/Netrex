use std::collections::HashMap;
use rakrs::*;

pub struct Server {
	// players on the server
	// change to actual player struct in the future
	players: HashMap<String, u8>,
}

impl Server {
	pub fn initialize() -> Self {
		Self {
			players: HashMap::new()
		}
	}

	pub fn tick(raknet_server: RakNetServer) {
		print!("Ticking...")
	}
}