use crate::network::session::SessionCommand;

use super::Server;

use mcpe_protocol::interfaces::VarString;
use mcpe_protocol::mcpe::Batch;
use mcpe_protocol::mcpe::Disconnect;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::time::sleep;

// todo clean this mess up it's stinky.
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
                match command {
                    SessionCommand::Disconnect(reason) => {
                        // check if the player exists.
                        if serv.players.read().unwrap().contains_key(&session) {
                            // remove the player from the server.
                            let mut lock = serv.players.write().expect("Failed to lock players");
                            lock.remove(&session);
                            drop(lock);
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
    }
    .await;
}
