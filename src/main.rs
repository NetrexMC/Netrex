pub mod network;
pub mod logger;
pub mod plugin;
pub mod util;
pub mod world;
pub mod server;

fn main() {
    let mut ticking_server = server::Server::initialize();
	// TODO -> Do config stuff for this!
	let net = network::initialize(&"0.0.0.0:19132");

	loop {
		// do tick
	}
}