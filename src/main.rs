#![feature(option_result_contains)]
pub mod logger;
pub mod network;
pub mod player;
pub mod plugin;
pub mod server;
pub mod util;
pub mod world;

use std::sync::Arc;
use std::sync::Mutex;

use server::start;
use server::Server;

#[tokio::main]
async fn main() {
    console_subscriber::init();
    let server = Arc::new(Mutex::new(Server::new()));
    start(Arc::clone(&server), "0.0.0.0:19132").await;
}
