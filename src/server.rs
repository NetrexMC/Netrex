use std::collections::HashMap;
use crate::logger::Logger;

pub struct Server {
	// players on the server
	// change to actual player struct in the future
	players: HashMap<String, u8>,
	logger: Logger
}

impl Server {
	pub fn initialize() -> Self {
		let mut string = String::new();
		string.push_str("Server");
		Self {
			players: HashMap::new(),
			logger: Logger::new(string)
		}
	}

	pub fn get_logger(&mut self) -> Logger {
		self.logger.clone()
	}

	pub fn get_players(&mut self) -> HashMap<String, u8> {
		self.players.clone()
	}
}