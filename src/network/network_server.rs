use rakrs::*;

pub struct NetworkServer {

}

pub fn initialize(address: &str) {
	let mut rak_server = RakNetServer::new(address.to_string());
}