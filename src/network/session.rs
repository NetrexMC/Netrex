
use std::collections::VecDeque;

use tokio::sync::mpsc::Sender;
use mcpe_protocol::mcpe::GamePacket;

#[derive(Debug, Clone)]
pub enum SessionCommand {
	Send(GamePacket),
	SendStream(Vec<u8>),
	Disconnect(String),
}

/// A network session keeps track of incoming and outgoing packets
/// This is mainly a proxy for the Server to better handle packets.
#[derive(Clone)]
pub struct Session {
	/// The address of the connection
	address: String,
	/// The packets that are queued, but not immediately sent.
	packets: VecDeque<GamePacket>,
	/// The sender to send packets to the client.
	/// This is a channel that is used to send packets to the client immediately.
	sender: Sender<SessionCommand>,
}

impl Session {
	/// Disconnect the session
	/// This will permanently remove the session from the server.
	pub async fn disconnect<T: Into<String>>(&self, reason: T) {
		self.sender.send(SessionCommand::Disconnect(reason.into())).await.expect("Failed to send disconnect command");
	}

	/// Ticks the session, this is called every tick
	/// This is used to send packets to the client
	pub async fn tick(&mut self) {
		// foreach packet in the packets queue, send it.
		// Packets should be batched and compressed here, but for now,
		// We just send them all at once.
		for packet in self.packets.drain(..) {
			// we don't care about this error.
			self.sender.send(SessionCommand::Send(packet)).await;
		}
	}

	/// Send a packet to the client
	/// If immediate is true, the packet will be sent immediately, completely skipping the queue.
	pub fn send(&mut self, packet: GamePacket, immediate: bool) {
		if immediate {
			self.sender.send(SessionCommand::Send(packet));
		} else {
			self.packets.push_back(packet);
		}
	}

	pub fn send_stream(&self, stream: Vec<u8>) {
		self.sender.send(SessionCommand::SendStream(stream));
	}

	pub fn address(&self) -> String {
		self.address.clone()
	}
}