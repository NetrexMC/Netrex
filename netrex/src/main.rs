use rak_rs::server::Listener;

mod network;
mod player;
mod plugin;
mod server;

#[tokio::main]
async fn main() {
	// load network layer
	let mut server = Listener::bind("0.0.0.0:19132").await.unwrap();

	server.start().await.unwrap();

	loop {
		let conn = server.accept().await.unwrap();
		println!("New connection from {}", conn.address);
	}
}