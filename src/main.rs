pub mod network;
pub mod plugin;
pub mod util;
pub mod world;
pub mod server;
use rakrs::*;

fn main() {
    let mut ticking_server = server::Server::initialize();
	// TODO -> Do config stuff for this!
	let mut raknet_server = RakNetServer::new(String::from("192.168.0.1:19132"));
	let handles = raknet_server.start();

	loop {
		ticking_server.tick(raknet_server);
	}
}