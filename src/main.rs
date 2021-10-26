pub mod logger;
pub mod network;
pub mod plugin;
pub mod server;
pub mod util;
pub mod world;

use std::sync::Mutex;

use server::Server;
use server::start;

fn main() {
	let mut server = Box::leak(Box::new(Mutex::new(Server::new())));
    start(server, &"0.0.0.0:19132");
}
