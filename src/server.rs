use crate::logger::Logger;
use crate::network::protocol::compression::decompress;
use crate::network::protocol::compression::compress;
use crate::network::session::{Session, SessionCommand};
use crate::player::{Player, PlayerData};
use binary_utils::*;
use mcpe_protocol::interfaces::{VarSlice, VarString};
use mcpe_protocol::mcpe::Disconnect;
use mcpe_protocol::mcpe::Batch;
use tokio::sync::mpsc::Sender;

use byteorder::ReadBytesExt;
use netrex_events::Channel;
use rakrs::{RakEvent, RakNetServer, RakResult};
use tokio::time::sleep;
use std::collections::{HashMap, VecDeque};
use std::io::Cursor;
use std::io::Write;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

macro_rules! make_batch {
	($batch: expr) => {{
		let mut buffer: Vec<u8> = vec![254];
		buffer.write_all(&compress(&mut $batch.fparse()).unwrap()).unwrap();
		buffer
	}}
}

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
        let mut lock = self.players.write().expect("Failed to lock players");
        if !lock.contains_key(&address) {
            // create a new session for the player
            self.logger
                .info(&format!("New player connected: {}", address));
            let session =
                Session::new(address.clone(), self.session_send.as_ref().unwrap().clone());
            let player = Player::new(session, PlayerData::unknown());
            lock.insert(address.clone(), player);
        }

        let player = lock.get_mut(&address).expect("Failed to get player");
		let batch = Batch::compose(&buffer, &mut 0);
        if let Ok(batch) = batch {
			for packet in batch.get_packets() {
				let res = player.handle(packet).await;
				if let Err(e) = res {
					self.logger
						.error(&format!("Failed to handle packet: {:?}", e));
				}
			}
		} else {
			self.logger.error(&format!("Failed to compose batch packet! {:?}", batch.err().unwrap()));
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
					logger.warn(&format!("{} disconnected but did not have a player to associate with: {}", address, reason));
				}

                None
            }
            RakEvent::GamePacket(address, buf) => {
                let mut buffer = buf.clone();
                let mut stream = Cursor::new(&mut buffer);
                let id = stream.read_u8().unwrap();
                let result = decompress(&buffer[1..]);

				println!("{}", id);

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

    let (task, server, channel) = rakrs::start(raknet, channel).await;
    s.lock().unwrap().network = Some(server);

    let ticking_server = Arc::clone(&s);

    println!("Starting raknet??!");
    futures::join!(task, spawn_schedulers(ticking_server, channel));
}

pub async fn spawn_schedulers(
    server: Arc<Mutex<Server>>,
    channel: Sender<(String, Vec<u8>, bool)>,
) {
    let (send, mut reciever) = tokio::sync::mpsc::channel::<(String, SessionCommand)>(2048);
    // tokio::spawn(async move {
    // 	loop {

    // 	}
    // });
    let mut serv = server.lock().unwrap();
    let sending = Arc::new(send);
    serv.session_send = Some(sending);
    drop(serv);
    async {
		let ticker_server = server.clone();
		tokio::spawn(async move {
			loop {
				sleep(Duration::from_millis(50)).await;
				let session_server = ticker_server.lock().unwrap();
				let mut players = session_server.players.write().unwrap();
				for (_, player) in players.iter_mut() {
					futures_executor::block_on(player.tick());
				}
			}
		});
		loop {
			if let Some(msg) = reciever.recv().await {
				let serv = server.lock().unwrap();
				let session = msg.0;
				let command = msg.1;
				println!("{:?}", command);
				match command {
					SessionCommand::Disconnect(reason) => {
						// check if the player exists.
						if serv.players.read().unwrap().contains_key(&session) {
							// disconnect the player
							let mut lock = serv.players.write().expect("Failed to lock players");
							lock.remove(&session);
							drop(lock);

							let mut batch = Batch::new(255);
							batch.add(Disconnect {
								hide_screen: false,
								message: VarString(reason),
							}.into());
							let res = channel
								.send((
									session.clone(),
									make_batch!(batch),
									true,
								))
								.await;

							if let Err(e) = res {
								println!("Failed to send disconnect packet: {:?}", e);
							}
						}
					}
					SessionCommand::Send(packet) => {
						if let Some(_player) = serv.players.write().unwrap().get_mut(&session) {
							let mut batch = Batch::new(255);
							batch.add(packet.into());

							let res = channel.send((session.clone(), make_batch!(batch), true)).await;

							if let Err(e) = res {
								println!("Failed to send packet: {:?}", e);
							}
						}
					}
					SessionCommand::SendBlk(mut packets, instant) => {
						if let Some(_player) = serv.players.write().unwrap().get_mut(&session) {
							let mut batches: VecDeque<Batch> = VecDeque::new();
							let mut current_batch = Batch::new(255);

							// this is easier on the memory usage
							let pks = packets.drain(..);

							for packet in pks {
								if current_batch.get_remaining() == 0 {
									batches.push_back(current_batch);
									current_batch = Batch::new(255);
								} else {
									current_batch.add(packet.into());
								}
							}

							for batch in batches {
								let res = channel.send((session.clone(), make_batch!(batch), instant)).await;

								if let Err(e) = res {
									println!("Failed to send packet: {:?}", e);
								}
							}
						}
					}
					SessionCommand::SendStream(pk) => {
						if serv.players.read().unwrap().contains_key(&session) {
							// disconnect the player
							let res = channel.send((session.clone(), pk, true)).await;

							if let Err(e) = res {
								println!("Failed to send disconnect packet: {:?}", e);
							}
						}
					}
					_ => {}
				}
			} else {
				continue;
			}
		}
    }.await;
}
