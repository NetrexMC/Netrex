pub mod handler;

use std::{collections::VecDeque, sync::Arc};

use async_trait::async_trait;
use mcpe_protocol::mcpe::Packet;
use tokio::sync::mpsc::{error::SendError, Sender};

use self::handler::{raw::RawHandler, HandlerError};

#[derive(Debug, Clone)]
pub enum SessionCommand {
    /// While not entirely immediate,
    /// this command is used to send packets to the player.
    Send(Packet),
    /// Sends all packets in the queue to the player
    /// A tuple of `(Packets, Instant?)`
    SendBlk(VecDeque<Packet>, bool),
    /// Immediate
    SendStream(Vec<u8>),
    /// Immediate
    Disconnect(String),
}

/// A network session keeps track of incoming and outgoing packets
/// This is mainly a proxy for the Server to better handle packets.
#[derive(Clone)]
pub struct Session {
    /// The address of the connection
    address: String,
    /// The packets that are queued, but not immediately sent.
    packets: VecDeque<Packet>,
    /// The sender to send packets to the client.
    /// This is a channel that is used to send packets to the client immediately.
    sender: Arc<Sender<(String, SessionCommand)>>,
}

impl Session {
    /// Create a new session for a new connection.
    /// This will create a new sender to send packets to the client.
    pub fn new(address: String, sender: Arc<Sender<(String, SessionCommand)>>) -> Self {
        Self {
            address,
            packets: VecDeque::new(),
            sender,
        }
    }
    /// Disconnect the session
    /// This will permanently remove the session from the server.
    pub async fn disconnect<T: Into<String>>(&self, reason: T) {
        self.dispatch(SessionCommand::Disconnect(reason.into()))
            .await
            .unwrap();
    }

    /// Ticks the session, this is called every tick
    /// This is used to send packets to the client
    pub async fn tick(&mut self) {
        // foreach packet in the packets queue, send it.
        // Packets should be batched and compressed here, but for now,
        // We just send them all at once.
        if self.packets.len() != 0 {
            self.dispatch(SessionCommand::SendBlk(self.packets.clone(), false))
                .await;
            self.packets.clear();
        }
    }

    /// Send a packet to the client
    /// If immediate is true, the packet will be sent immediately, completely skipping the queue.
    pub async fn send(&mut self, packet: Packet, immediate: bool) {
        if immediate {
            self.dispatch(SessionCommand::Send(packet)).await;
        } else {
            self.packets.push_back(packet);
        }
    }

    /// Immediately sends any buffer to the client
    pub async fn send_stream(&self, stream: Vec<u8>) {
        self.dispatch(SessionCommand::SendStream(stream)).await;
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }

    /// Handles a raw payload and retrieves a packet from it.
    pub async fn handle_raw(
        &self,
        interface: &mut dyn SessionInterface,
        buffer: Vec<u8>,
    ) -> Result<(), HandlerError> {
        if let Ok(batch) = RawHandler::recv(buffer).await {
            for packet in batch.get_packets() {
                interface.on_packet(packet).await?;
            }
        }

        Ok(())
    }

    async fn dispatch(
        &self,
        command: SessionCommand,
    ) -> Result<(), SendError<(String, SessionCommand)>> {
        return self.sender.send((self.address.clone(), command)).await;
    }
}

/// This trait is used to implement multiple interfaces for a Session.
/// Each interface must be able to handle packets from a session.
#[async_trait]
pub trait SessionInterface {
    /// Handles a packet from a session.
    /// This is called every time a packet is received from the session.
    async fn on_packet(&mut self, packet: Packet) -> Result<(), HandlerError>;
}
