pub mod network;
pub mod logger;
pub mod plugin;
pub mod util;
pub mod world;
pub mod server;

use server::Server;

fn main() {
	let mut server = Server::new();
	server.start(&"0.0.0.0:19132");
}