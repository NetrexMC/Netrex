pub mod logger;
pub mod network;
pub mod plugin;
pub mod server;
pub mod util;
pub mod world;

use server::Server;

fn main() {
	let mut server = Server::new();
	server.start(&"0.0.0.0:19132");
}
