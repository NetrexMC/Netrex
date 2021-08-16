use std::io;
use std::io::prelude::*;
use flate2::read::DeflateDecoder;
use flate2::write::DeflateEncoder;
use flate2::Compression;
use binary_utils::{BinaryStream, IBinaryStream};

pub fn decompress(raw_buf: &[u8]) -> io::Result<BinaryStream> {
	let mut ret_data: Vec<u8> = Vec::new();
	let mut reader = DeflateDecoder::new(raw_buf);

	match reader.read_to_end(&mut ret_data) {
		Ok(_) => Ok(BinaryStream::init(&ret_data)),
		Err(v) => Err(v)
	}
}

pub fn compress(buf: &mut [u8]) -> io::Result<BinaryStream> {
	let mut writer = DeflateEncoder::new(Vec::new(), Compression::best());
	let written = writer.write_all(buf);
	if written.is_err() {
		Err(written.unwrap_err())
	} else {
		match writer.finish() {
			Ok(v) => Ok(BinaryStream::init(&v)),
			Err(v) => Err(v)
		}
	}
}