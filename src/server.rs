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

    pub fn receive(address: String, buffer: Vec<u8>) {}

    pub fn start(&mut self, address: &str) {
        let mut raknet = RakNetServer::new(address.to_string());
        let ref_rak = Arc::new(&self);
        let mut logger_cloned = self.get_logger();

        self.logger.info("Starting Server");
        let (_send, _work) = raknet_start!(raknet, move |event: &RakNetEvent| {
            match event.clone() {
                RakNetEvent::Disconnect(address, reason) => {
                    logger_cloned.info(
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
                    logger_cloned.info(
                        &format!("Client[{}] sent packet: {:?}", address, frames[0][0]).to_string()
                            [..],
                    );
                    Some(RakResult::Motd(Motd::default()))
                }
                _ => None,
            }
        });
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
