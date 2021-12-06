use crate::logger::Logger;
use crate::network::protocol::compression::decompress;
use crate::network::protocol::mcbe::login::do_login;
use binary_utils::*;
use mcpe_protocol::interfaces::VarSlice;

use byteorder::ReadBytesExt;
use mcpe_protocol::mcpe::{construct_packet, GamePacket};
use netrex_events::Channel;
use rakrs::raknet_start;
use rakrs::{Motd, RakNetEvent, RakNetServer, RakResult};
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{Arc, Mutex};

macro_rules! exp {
    ($e: expr) => {
        ::std::sync::Arc::make_mut(&mut $e)
    };
}
pub struct Server {
    /// A Hashmap of players connected to the server.
    pub sessions: HashMap<String, u8>,
    pub logger: Logger,
    pub network: Option<RakNetServer>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            logger: Logger::new("Server".to_owned()),
            network: None,
        }
    }

    pub fn receive(&mut self, address: String, buffer: Vec<u8>) {
        let mut buf = Cursor::new(&buffer);
        // get the id of the packet
        let id = buf.read_u8().unwrap();

        let packet: GamePacket = construct_packet(id, &buffer[1..]);
        // dropped bytes
        // println!("Dropped bytes: {:?}\n", &buffer[5..45]);

        match packet {
            GamePacket::Login(pk) => {
                println!("{:?}", &pk.protocol);
                // let data = deconstruct(pk);
                // dbg!(data);
                do_login(self, address, pk);
            }
            _ => return,
        }
    }

    pub fn get_logger(&mut self) -> Logger {
        self.logger.clone()
    }

    pub fn get_players(&mut self) -> HashMap<String, u8> {
        self.sessions.clone()
    }
}

pub fn start(server: Arc<Mutex<Server>>, address: &str) {
    let server_thread = Arc::clone(&server);
    let mut raknet = RakNetServer::new(address.to_string());
    let mut s = server.lock().unwrap();
    let mut logger = Arc::new(s.get_logger().clone());
    drop(s);

    let mut logger_thread = Arc::clone(&logger);

    exp!(logger).info("Starting Server");

	let mut channel = Channel::<RakNetEvent, RakResult>::new();
	let mut listener = |event: RakNetEvent, result: Option<RakResult>| -> Option<RakResult> {
        match event.clone() {
            RakNetEvent::Disconnect(address, reason) => {
                exp!(logger_thread)
                    .info(&format!("{} disconnected due to: {}", address, reason).to_string()[..]);
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

                    let s: &Vec<u8> = &VarSlice::fcompose(&decompressed[position..], &mut position)
                        .0
                        .clone();
                    dstream.set_position(position as u64);
                    frames.push(s.to_vec());
                }
                let mut serv = server_thread.lock().expect("not cool!");
                for frame in frames {
                    // func(address.clone(), frame);
                    // get the connection
                    serv.receive(address.clone(), frame);
                }
                drop(serv);
                Some(RakResult::Motd(Motd::default()))
            }
            _ => None,
        }
    };
	channel.receive(&mut listener);
    let threads = raknet_start!(raknet, channel);
    exp!(logger).info("RakNet Started.");
    let mut serv = server.as_ref().lock().unwrap();
    serv.network = Some(raknet);
    drop(serv);
    exp!(logger).info("Server started!");

    // loop {
    // 	if let Ok(mut serv) = server.try_lock() {
    //     	serv.tick();
    // 		drop(serv);
    // 	} else {
    // 		// if the tick fails, infinitely retry until we're able to do so
    // 		loop {
    // 			// this will hang if this errors
    // 			if let Ok(mut serv) = server.try_lock() {
    // 				println!("Saved tick!");
    // 				serv.tick();
    // 				drop(serv);
    // 				break;
    // 			}
    // 		}
    // 	}
    // }
    // server ticking is bad.
}
