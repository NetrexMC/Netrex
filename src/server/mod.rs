mod schedulers;
use schedulers::*;

use crate::logger::Logger;
use crate::network::session::{Session, SessionCommand};
use crate::player::{Player, PlayerData};
use tokio::sync::mpsc::Sender;

use netrex_events::Channel;
use rakrs::{RakEvent, RakNetServer, RakResult};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

pub struct Server {
    /// A Hashmap of players connected to the server.
    pub players: RwLock<HashMap<String, Player>>,
    pub logger: Logger,
    pub network: Option<Arc<RakNetServer>>,
    pub session_send: Option<Arc<Sender<(String, SessionCommand)>>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            logger: Logger::new("Server".to_owned()),
            network: None,
            players: RwLock::new(HashMap::new()),
            session_send: None,
        }
    }

    pub async fn receive(&mut self, address: String, buffer: Vec<u8>) {
        // this is a bit hacky but it works
        let mut lock = self.players.write().expect("Failed to lock players");
        if !lock.contains_key(&address) {
            // create a new session for the player
            self.logger
                .info(&format!("New player connected: {}", address));
            let sender = self.session_send.as_ref().unwrap().clone();
            let session = Session::new(address.clone(), sender);
            let player = Player::new(session, PlayerData::unknown());
            lock.insert(address.clone(), player);
        }

        let player = lock.get_mut(&address).expect("Failed to get player");
        // handle the packet through the players session
        let handled = player.handle_raw(buffer.clone()).await;

        if let Err(e) = handled {
            if e.is_login_error() {
                player.disconnect(e.to_string()).await
            }
            println!("[{}] Could not handle a packet from player! {}", address, e);
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
                let serv = ref_server.lock().expect("not cool!");

                let mut players = serv.players.write().unwrap();
                if players.contains_key(&address) {
                    logger.info(&format!("{} disconnected due to: {}", address, reason));
                    players.remove(&address);
                } else {
                    logger.warn(&format!(
                        "{} disconnected but did not have a player to associate with: {}",
                        address, reason
                    ));
                }

                None
            }
            RakEvent::GamePacket(address, buf) => {
                let mut serv = ref_server.lock().expect("not cool!");
                futures_executor::block_on(serv.receive(address.clone(), buf));
                drop(serv);
                None
            }
            _ => None,
        }
    };

    channel.receive(&mut packet_listener);

    let (task, server, channel) = rakrs::start(raknet, channel).await;
    s.lock().unwrap().network = Some(server);

    let ticking_server = Arc::clone(&s);

    println!("Starting raknet??!");
    futures::join!(task, spawn_schedulers(ticking_server, channel));
}
