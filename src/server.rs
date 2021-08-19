use crate::logger::Logger;
use crate::network::protocol::compression::decompress;
use binary_utils::{BinaryStream, IBinaryStream, IBufferRead};
use rakrs::{Motd, RakNetEvent, RakNetServer, SERVER_ID, RakEventListenerFn};
use rakrs::conn::{Connection, RecievePacketFn};
use std::collections::HashMap;
use std::sync::Arc;

pub struct Server {
	// players on the server
	// change to actual player struct in the future
	players: HashMap<String, u8>,
	logger: Logger,
	network: Option<RakNetServer>,
}

impl Server {
	pub fn new() -> Self {
		Self {
			players: HashMap::new(),
			logger: Logger::new("Server".to_owned()),
			network: None,
		}
	}

	pub fn start(&mut self, address: &str) {
		let mut raknet = RakNetServer::new(address.to_string());
		let ref_rak = Arc::new(&raknet);
		let mut logger_cloned = self.get_logger();
		self.logger.info("Starting Server");

		let packet_handler: Arc<RecievePacketFn> =  Arc::new(move |_conn: &mut Connection, stream: &mut BinaryStream| {
			stream.read_byte();
			let result = decompress(&stream.get_buffer()[stream.get_offset()..]);

			if result.is_err() {
				println!(
					"Something when wrong when decoding: {}",
					result.unwrap_err()
				);
				return;
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

		let event_handler: Box<RakEventListenerFn> = Box::new(move |event: &RakNetEvent| {
			match event.clone() {
				RakNetEvent::Disconnect(address, reason) => {
					logger_cloned.info(&format!("{} disconnected due to: {}", address, reason).to_string()[..]);
				},
				_ => return
			}
		});
		let (_send, _work) = raknet.start(packet_handler, event_handler);
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

	fn tick(&mut self) {}
}
