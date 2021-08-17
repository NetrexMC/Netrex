use std::collections::HashMap;
use std::sync::Arc;
use rakrs::{ RakNetServer, conn::Connection, RakNetEvent, Motd, SERVER_ID };
use binary_utils::{BinaryStream, IBinaryStream, IBufferRead};
use crate::network::protocol::compression::decompress;
use crate::logger::Logger;

pub struct Server {
	// players on the server
	// change to actual player struct in the future
	players: HashMap<String, u8>,
	logger: Logger,
	network: Option<RakNetServer>
}

impl Server {
	pub fn new() -> Self {
		Self {
			players: HashMap::new(),
			logger: Logger::new("Server".to_owned()),
			network: None
		}
	}

	pub fn start(&mut self, address: &str) {
		let mut raknet = RakNetServer::new(address.to_string());
		let ref_rak = Arc::new(&raknet);
		self.logger.info("Starting Server");

		raknet.set_reciever(move |_conn: &mut Connection, stream: &mut BinaryStream| {
			stream.read_byte();
			let result = decompress(&stream.get_buffer()[stream.get_offset()..]);

			if result.is_err() {
				println!("Something when wrong when decoding: {}", result.unwrap_err());
				return
			}

			let mut decompressed = result.unwrap();
			let mut frames = Vec::new();
			loop {
				if decompressed.get_offset() == decompressed.get_length() {
					break;
				}
				let size: usize = decompressed.read_uvar_int() as usize;
				let slice = BinaryStream::init(&decompressed.read_slice_exact(Some(size)));
				frames.push(slice);
			}
			println!("Packet ID: {}", frames[0].read_byte())
		});

		let (_send, _work) = raknet.start();
		self.logger.info("RakNet Started.");
		self.network = Some(raknet);
		self.logger.info("Server started!");

		loop {
			self.tick();
		}
	}

	pub fn get_logger(&mut self) -> Logger {
		self.logger.clone()
	}

	pub fn get_players(&mut self) -> HashMap<String, u8> {
		self.players.clone()
	}

	fn tick(&mut self) {

	}
}