use rakrs::{ RakNetServer, conn::Connection };
use binary_utils::{BinaryStream, IBinaryStream, IBufferRead};
use super::compression::decompress;

pub fn initialize(address: &str) {
	let mut rak_server = RakNetServer::new(address.to_string());
	rak_server.set_reciever(handle_test);
	let (r, s) = rak_server.start();
	r.join().unwrap();
	s.join().unwrap();
}

fn handle_test(_conn: &mut Connection, stream: &mut BinaryStream) {
	stream.set_offset(0);
	stream.read_byte();
	let result = decompress(&stream.get_buffer()[stream.get_offset()..]);

	if result.is_err() {
		println!("Something when wrong when decoding: {}", result.unwrap_err());
		return
	}

	let mut decompressed = result.unwrap();

	let mut frames = Vec::new();
	loop {

		if decompressed.get_offset() == decompressed.get_length() {
			break;
		}
		let size: usize = decompressed.read_uvar_int() as usize;
		let slice = BinaryStream::init(&decompressed.read_slice_exact(Some(size)));

		frames.push(slice);
	}

	println!("Packet ID: {}", frames[0].read_byte())
}