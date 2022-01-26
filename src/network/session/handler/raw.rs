use std::io::Cursor;
use std::io::Write;

use binary_utils::Streamable;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use mcpe_protocol::mcpe::Batch;
use mcpe_protocol::mcpe::Packet;

use crate::network::compression::compress;
use crate::network::compression::decompress;

use super::HandlerError;

/// Errors for raw handler
#[derive(Debug)]
pub enum RawHandlerError {
    UnknownPacket(String),
    PacketLimitExceeded,
    InvalidPacket,
}

/// A raw packet handler, handles game decoding and encoding.
/// This is the most basic handler, it just passes the packet to the correct handler.
/// This is used for the client to handle packets from the server.
pub struct RawHandler;

impl RawHandler {
    /// This function is called the moment any buffer is recieved from
    /// the client. It will decode the buffer and return a batched packet.
    pub async fn recv(buffer: Vec<u8>) -> Result<Batch, HandlerError> {
        let mut stream = Cursor::new(buffer.clone());

        if stream.read_u8()? != 254 {
            // This is not a minecraft packet!
            // We need to throw an error!
            return Err(RawHandlerError::InvalidPacket.into());
        }

        // Lets decompress the buffer.
        let game_buffer = &buffer[1..];
        let decompress_res = decompress(&game_buffer);

        if let Err(err) = decompress_res {
            return Err(HandlerError::UnknownError(format!(
                "Decompression error: {}",
                err
            )));
        }

        let decompressed = decompress_res?;
        // Get the packets from the buffer.
        let batched = Batch::compose(&decompressed[..], &mut 0)?;

        // assert the limit.
        if batched.get_size() > 255 {
            return Err(RawHandlerError::PacketLimitExceeded.into());
        }

        Ok(batched)
    }

    /// This will process and "batch" every packet in the queue.
    /// This is called by the server when it's ready to send packets.
    /// This will return a Vector of Byte streams `Vec<u8>`, representing
    /// Each batch packet.
    pub async fn send(packets: Vec<Packet>) -> Result<Vec<Vec<u8>>, HandlerError> {
        let mut batches: Vec<Vec<u8>> = Vec::new();
        let mut current_batch = Batch::new(255);

        for packet in packets {
            if current_batch.get_remaining() == 0 {
                // we need to parse the batch packet.
                let mut buffer = current_batch.parse()?;
                // now let's compress the buffer.
                let compressed = compress(&mut buffer[..])?;
                // add the compressed buffer to the batches.
                batches.push(Self::write_header(compressed));
                // batches.push(compress(&mut Self::write_header(buffer)[..])?);

                // reset the current batch
                current_batch = Batch::new(255);
            }

            current_batch.add(packet);
        }

        // parse the last batch.
        let mut buffer = current_batch.parse()?;
        let compressed = compress(&mut buffer[..])?;
        println!(
            "\nDecompressed data for memes: {:?}\n\n",
            decompress(&compressed[..])
        );
        batches.push(Self::write_header(compressed));
        // batches.push(compress(&mut Self::write_header(buffer)[..])?);

        Ok(batches)
    }

    /// Internal utility function to quickly write the mcpe game packet to the buffer
    fn write_header(buffer: Vec<u8>) -> Vec<u8> {
        let mut header: Vec<u8> = Vec::new();
        header.write_u8(254).unwrap();
        header.write_all(&buffer);
        header
    }
}
