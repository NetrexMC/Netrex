use crate::logger::Logger;
use crate::network::protocol::compression::decompress;
use binary_utils::*;
use mcpe_protocol::interfaces::{Slice, VarSlice};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use rakrs::conn::Connection;
use rakrs::raknet_start;
use rakrs::{Motd, RakEventListenerFn, RakNetEvent, RakNetServer, RakResult, SERVER_ID};
use std::collections::HashMap;
use std::io::{Cursor, Read, Write};
use std::sync::{Arc, Mutex};

pub struct Server {
    // players on the server
    // change to actual player struct in the future
    pub players: HashMap<String, u8>,
    pub logger: Logger,
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

    pub fn recieve(&mut self, address: String, buffer: Vec<u8>) {}

    pub fn get_logger(&mut self) -> Logger {
        self.logger.clone()
    }

    pub fn get_players(&mut self) -> HashMap<String, u8> {
        self.players.clone()
    }

    fn tick(&mut self) {}
}

pub fn start(server: &'static mut Mutex<Server>, address: &str) {
		let mut raknet = RakNetServer::new(address.to_string());
		let s = server.lock().unwrap();
        let mut logger = s.get_logger().clone();
		let server_clone = Arc::new(&server);
		drop(s);
		// let reciever = Box::new(server.receive);

        logger.info("Starting Server");
        let threads = raknet_start!(raknet, move |event: &RakNetEvent| {
            match event.clone() {
                RakNetEvent::Disconnect(address, reason) => {
                    logger.info(
                        &format!("{} disconnected due to: {}", address, reason).to_string()[..],
                    );
                    None
                }
                RakNetEvent::GamePacket(address, buf) => {
                    let mut buffer = buf.clone();
                    let mut stream = Cursor::new(&mut buffer);
                    stream.read_u8().unwrap();
                    let result = decompress(&buffer[1..]);

                    if result.is_err() {
                        println!(
                            "Something when wrong when decoding: {}",
                            result.unwrap_err()
                        );
                        return None;
                    }
                    let decompressed = &result.unwrap();
                    let mut dstream = Cursor::new(decompressed);
                    let mut frames = Vec::<Vec<u8>>::new();
                    loop {
                        if dstream.position() as usize >= decompressed.len() {
                            break;
                        }
                        let mut position: usize = dstream.position() as usize;
                        let s = &VarSlice::compose(&decompressed[position..], &mut position)
                            .0
                            .clone();
                        dstream.set_position(position as u64);
                        frames.push(s.to_vec());
                    }
                    logger.info(
                        &format!("Client[{}] sent packet: {:?}", address, frames[0][0]).to_string()
                            [..],
                    );
					let serv = server.lock().expect("Uh oh...");
					for frame in frames {
						// func(address.clone(), frame);
						serv.recieve(address.clone(), frame);
					}
					drop(serv);
                    Some(RakResult::Motd(Motd::default()))
                }
                _ => None,
            }
        });
        logger.info("RakNet Started.");
		let serv = server.lock().unwrap();
		serv.network = Some(raknet);
        drop(serv);
        logger.info("Server started!");

        loop {
			if let Ok(serv) = server.try_lock() {
            	serv.tick();
				drop(serv)
			} else {
				
			}
        }
		drop(server);
}