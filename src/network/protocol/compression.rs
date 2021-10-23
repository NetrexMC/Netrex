use flate2::read::DeflateDecoder;
use flate2::write::DeflateEncoder;
use flate2::Compression;
use std::io;
use std::io::prelude::*;

pub fn decompress(raw_buf: &[u8]) -> io::Result<Vec<u8>> {
    let mut ret_data: Vec<u8> = Vec::new();
    let mut reader = DeflateDecoder::new(raw_buf);

    match reader.read_to_end(&mut ret_data) {
        Ok(_) => Ok(ret_data.clone()),
        Err(v) => Err(v),
    }
}

pub fn compress(buf: &mut [u8]) -> io::Result<Vec<u8>> {
    let mut writer = DeflateEncoder::new(Vec::new(), Compression::best());
    let written = writer.write_all(buf);
    if written.is_err() {
        Err(written.unwrap_err())
    } else {
        match writer.finish() {
            Ok(v) => Ok(v),
            Err(v) => Err(v),
        }
    }
}
