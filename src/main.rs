pub mod network;
pub mod logger;
pub mod plugin;
pub mod util;
pub mod world;
pub mod server;

fn main() {
	server::Server::initialize();
	// TODO -> Do config stuff for this!
	network::initialize(&"0.0.0.0:19132");

	loop {
		// do tick
	}
}