use crate::logger::Logger;
use crate::network::protocol::compression::decompress;
use crate::network::session::Session;
use crate::player::{Player, PlayerData};
use binary_utils::*;
use mcpe_protocol::interfaces::VarSlice;

use byteorder::ReadBytesExt;
use netrex_events::Channel;
use rakrs::{RakEvent, RakNetServer, RakResult};
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{Arc, Mutex, RwLock};

pub struct Server {
    /// A Hashmap of players connected to the server.
    pub players: RwLock<HashMap<String, Player>>,
    pub logger: Logger,
    pub network: Option<Arc<RakNetServer>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            logger: Logger::new("Server".to_owned()),
            network: None,
            players: RwLock::new(HashMap::new()),
        }
    }

    pub async fn receive(&mut self, address: String, buffer: Vec<u8>) {
        let mut lock = self.players.write().expect("Failed to lock players");
        if !lock.contains_key(&address) {
            // create a new session for the player
            self.logger
                .info(&format!("New player connected: {}", address));
            let session = Session::new(address.clone());
            let player = Player::new(session, PlayerData::unknown());
            lock.insert(address.clone(), player);
        }

        let player = lock.get_mut(&address).expect("Failed to get player");
        let res = player.handle(buffer).await;
        if let Err(e) = res {
            self.logger
                .error(&format!("Failed to handle packet: {:?}", e));
			player.session.disconnect("Failed to handle packet").await;
		}
    }

    pub fn get_logger(&mut self) -> Logger {
        self.logger.clone()
    }
}

pub async fn start<Add: Into<String>>(s: Arc<Mutex<Server>>, address: Add) {
    let raknet = RakNetServer::new(address.into());
    let channel = Channel::<RakEvent, RakResult>::new();
    let ref_server = Arc::clone(&s);
    let mut logger = s.lock().unwrap().get_logger().clone();
    let mut packet_listener = |event: RakEvent, _: Option<RakResult>| -> Option<RakResult> {
        match event.clone() {
            RakEvent::Disconnect(address, reason) => {
                logger
                    .info(&format!("{} disconnected due to: {}", address, reason).to_string()[..]);
                None
            }
            RakEvent::GamePacket(address, buf) => {
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
                let mut serv = ref_server.lock().expect("not cool!");
                for frame in frames {
                    // func(address.clone(), frame);
                    // get the connection
                    // todo: REMOVE THIS HACK LOL
                    futures_executor::block_on(serv.receive(address.clone(), frame));
                }
                drop(serv);
                None
            }
            _ => None,
        }
    };

    channel.receive(&mut packet_listener);

    let (task, server) = rakrs::start(raknet, channel).await;
    s.lock().unwrap().network = Some(server);

    println!("Starting raknet??!");
    task.await;
}
