use crate::ack::is_ack_or_nack;
use crate::ack::{queue::AckQueue, queue::NAckQueue, Ack, Record};
use crate::fragment::{FragmentList, FragmentStore};
use crate::frame::{Frame, FramePacket};
use crate::online::{handle_online, OnlinePackets, log_online};
use crate::protocol::offline::*;
use crate::reliability::{Reliability, ReliabilityFlag};
use crate::util::tokenize_addr;
use crate::{Motd, RakEvent};
use binary_utils::*;
use byteorder::ReadBytesExt;
use std::collections::VecDeque;
use std::fmt::Display;
use std::io::Cursor;
use std::net::SocketAddr;
use std::time::SystemTime;

pub type RecievePacketFn = fn(&mut Connection, &mut Vec<u8>);

#[derive(Clone, Debug, PartialEq)]
pub enum ConnectionState {
    Connecting,
    Connected,
    Disconnected,
    TimingOut,
    Offline,
}

impl ConnectionState {
    /// Whether or not the ConnectionState is `Disconnected`.
    pub fn is_disconnected(&self) -> bool {
        match *self {
            Self::Disconnected => true,
            _ => false,
        }
    }

    /// Whether or not the ConnectionState is `Connected`.
    pub fn is_connected(&self) -> bool {
        match *self {
            Self::Connected => true,
            _ => false,
        }
    }

    /// Whether or not the Connection is:
    /// - **Offline**
    /// - **TimingOut**
    pub fn is_unavailable(&self) -> bool {
        match *self {
            Self::Offline | Self::TimingOut => true,
            _ => false,
        }
    }

    /// Whether or not the Connection can reliably recieve
    /// a buffer, the states that return true are:
    /// - **Connected**
    /// - **Connecting**
    /// - **Disconnected**
    pub fn is_reliable(&self) -> bool {
        match *self {
            Self::Disconnected | Self::Connected | Self::Connecting => true,
            _ => false,
        }
    }
}

impl Display for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Connecting => write!(f, "Connecting"),
            Self::Connected => write!(f, "Connected"),
            Self::Disconnected => write!(f, "Disconnected"),
            Self::TimingOut => write!(f, "TimingOut"),
            Self::Offline => write!(f, "Offline"),
        }
    }
}

macro_rules! conn_create_error_event {
    ($self: expr, $buffer: expr, $message: expr) => {
        $self.event_dispatch.push_back(RakEvent::ComplexBinaryError(
            $self.address_token.clone(),
            $buffer,
            $message,
        ))
    };
}

/// Connection struct.
/// Connections are used internally by RakNet to manage connecting clients.
/// > It is important to not hold a reference to this struct as it is thread blocking!
/// > This is because the `Connection` struct is used within a mutex and can cause a deadlock.
/// ```rust no_run
/// use rakrs::conn::Connection;
/// use rakrs::util::from_tokenized;
/// fn main() {
///     // create a connection!
///     let conn = Connection::new(from_tokenized("192.168.0.1:25565"), 0, Motd::new());
/// }
/// ```
#[derive(Clone)]
pub struct Connection {
    /// The address the client is connected with.
    pub address: SocketAddr,
    /// The address of the client, however it's tokenized.
    pub address_token: String,
    /// The start time of the `RakNetServer`.
    pub time: SystemTime,
    /// The **Max transfer unit** for the client.
    /// Outbound buffers will be reduced to this unit.
    pub mtu_size: u16,
    /// The state of the given connection.
    /// States include:
    /// - **Connecting**: Client is not connected, but is performing connection sequence.
    /// - **Connected**: Client has performed connection sequence and is reliable.
    /// - **Disconnected**: The client is sending information, but is not connected to the server.
    /// - **Offline**: We have stopped recieving responses from the client.
    pub state: ConnectionState,
    /// A list of events to be emitted on next tick.
    pub event_dispatch: VecDeque<RakEvent>,
    /// The last time the client has sent something to the server, that was a connected packet.
    pub recv_time: SystemTime,
    /// A Vector of streams to be sent.
    /// This should almost always be a Frame, with exceptions
    /// to offline packets.
    pub send_queue: VecDeque<Vec<u8>>,
    /// A list of buffers that exceed the MTU size
    /// This queue will be shortened into individual fragments,
    /// and sent to the client as fragmented frames.
    send_queue_large: VecDeque<Vec<u8>>,
    /// Stores the fragmented frames by their
    /// `frame_index` value from a given packet.
    /// When a `FrameList` is ready from a `FragmentStore` it's assembled
    /// into a `FramePacket` which can then be added to the `send_queue`.
    fragmented: FragmentStore,
    /// Stores the next available fragment id.
    /// This variable will reset after the sequence
    /// containing the fragment id's we sent has been
    /// acknowledged by the client.
    ///
    /// However in the event this never occurs, fragment id will reset after
    /// it reaches `65535` as a value
    fragment_id: u16,
    /// The last recieved sequence id
    recv_seq: u32,
    /// The last send sequence id used
    send_seq: u32,
    /// The ACK queue (packets we got)
    ack: AckQueue,
    /// The NACK queue (Packets we didn't get)
    nack: NAckQueue,
    /// The Motd reference.
    pub motd: Motd,
    /// The server GUID.
    pub server_guid: u64,
}

impl Connection {
    pub fn new(
        address: SocketAddr,
        start_time: SystemTime,
        server_guid: u64,
        port: String,
    ) -> Self {
        Self {
            address,
            address_token: tokenize_addr(address),
            time: start_time,
            recv_time: SystemTime::now(),
            mtu_size: 2048,
            state: ConnectionState::Disconnected,
            event_dispatch: VecDeque::new(),
            // recv,
            send_queue: VecDeque::new(),
            send_queue_large: VecDeque::new(),
            fragmented: FragmentStore::new(),
            recv_seq: 0,
            send_seq: 0,
            fragment_id: 0,
            ack: AckQueue::new(),
            nack: NAckQueue::new(),
            motd: Motd::new(server_guid, port),
            server_guid,
        }
    }

    /// Send a binary stream to the specified client. (except it will be framed.)
    pub fn send(&mut self, stream: Vec<u8>, instant: bool) {
        if instant {
            let mut frame_packet = FramePacket::new();
            let mut frame = Frame::init();
            frame.reliability = Reliability::new(ReliabilityFlag::Unreliable);
            frame.body = stream.clone();
            frame_packet.seq = self.next_send_seq().into();
            frame_packet.frames.push(frame);
            let final_frame = frame_packet.parse();
            if final_frame.is_err() {
                // we couldn't send this frame
                // we need to communicate this to the server
                conn_create_error_event!(
                    self,
                    stream.clone(),
                    final_frame.err().unwrap().get_message()
                );
            } else {
				if cfg!(any(test, feature = "dbg")) {
					log_online(format!("[{}] Sent packet: {:#?}\n", self.address_token, &frame_packet));
				}
                self.send_queue.push_back(final_frame.ok().unwrap());
            }
        } else {
            self.send_queue_large.push_back(stream);
        }
    }

    /// Send a binary stream to the specified client.
    /// Except it will be raw.
    pub fn send_stream(&mut self, stream: Vec<u8>, instant: bool) {
        if instant {
            self.send_queue.push_back(stream);
        } else {
            self.send_queue_large.push_back(stream);
        }
    }

    /// The recieve handle for a connection.
    /// This is called when RakNet parses any given byte buffer from the socket.
    pub fn recv(&mut self, buf: &Vec<u8>) {
        let mut stream = Cursor::new(buf);
        // Update the recieve time.
        self.recv_time = SystemTime::now();

        if self.state.is_disconnected() {
            let pk = OfflinePackets::from_byte(stream.read_u8().unwrap());
            let handler = handle_offline(self, pk, stream.get_mut());
            match handler {
                Ok(buffer) => self.send_stream(buffer, true),
                Err(error) => {
                    // for some reason we failed to read this packet
                    // again, we need to communicate this to the server
                    conn_create_error_event!(self, buf.clone(), error.get_message())
                }
            }
        } else if self.state.is_reliable() {
            // this packet is almost always a frame packet
            let online_packet = OnlinePackets::from_byte(stream.read_u8().unwrap());

            if is_ack_or_nack(online_packet.to_byte()) {
                stream.set_position(0);
                return self.handle_ack(stream.get_ref().clone());
            }

            match online_packet {
                OnlinePackets::Disconnect => {
                    self.state = ConnectionState::Offline;
                    self.event_dispatch.push_back(RakEvent::Disconnect(
                        tokenize_addr(self.address),
                        "Client disconnect".to_owned(),
                    ));
                    return;
                }
                OnlinePackets::FramePacket(_) => {
                    let frame_packet =
                        FramePacket::compose(stream.get_ref(), &mut (stream.position() as usize));

                    if frame_packet.is_err() {
                        conn_create_error_event!(
                            self,
                            buf.clone(),
                            frame_packet.err().unwrap().get_message()
                        );
                    } else {
                        self.handle_frames(&mut frame_packet.ok().unwrap());
                    }
                    return;
                }
                _ => {}
            }
        } else {
            // if we're not connected or disconnected, we need to handle the packet
            // as an offline packet
            if !self.state.is_reliable() {
                // println!("[Client {}] is in {} state (unreliable). But sent a packet, setting state to disconnected due to anomaly.", self.address, self.state);
                self.state = ConnectionState::Disconnected;
            }
        }
    }

    /// When the client sends an **Acknowledge**, we check:
    /// - If we have already recieved this packet.
    ///   If so, we respectfully ignore the packet.
    ///
    /// - The "records" in the acknowledge packet.
    ///   We iterate through the records, and if
    ///   any record sequence **does not exist**
    ///   we add this sequence number to the **Nack** queue,
    ///   which is then sent to the client when the connection ticks
    ///   to *hopefully* force the client to eventually send that packet.
    pub fn handle_ack(&mut self, packet: &Vec<u8>) {
        if let Ok(got) = Ack::compose(&packet[..], &mut 0) {
            for record in got.records {
                if record.is_single() {
                    let sequence = match record {
                        Record::Single(rec) => rec.sequence,
                        _ => continue,
                    };

                    if !self.ack.has_seq(sequence) {
                        self.nack.push_seq(sequence);
                    }
                } else {
                    let range = match record {
                        Record::Range(rec) => rec,
                        _ => continue,
                    };

                    let sequences = range.get_sequences();

                    for sequence in sequences {
                        if !self.ack.has_seq(sequence) {
                            self.nack.push_seq(sequence);
                        }
                    }
                }
            }
        } else {
            conn_create_error_event!(
                self,
                packet.clone(),
                "Failed reading ack packet.".to_string()
            );
        }
    }

    /// Iterates over every `Frame` of the `FramePacket` and does the following checks:
    /// - Checks if the frame is fragmented, if it is,
    ///   we check if all fragments have been sent to the server.
    ///   If all packets have been sent, we "re-assemble" them.
    ///   If not, we simply add the fragment to a fragment list,
    ///   and continue to the next frame
    ///
    /// - If it is not fragmented, we handle the frames body. (Which should contain a valid RakNet payload)
    pub fn handle_frames(&mut self, frame_packet: &mut FramePacket) {
        self.recv_seq = frame_packet.seq.into();
        self.ack
            .push_seq(frame_packet.seq.into(), frame_packet.fparse());
        for frame in frame_packet.frames.iter_mut() {
            if frame.fragment_info.is_some() {
                // the frame is fragmented!
                self.fragmented.add_frame(frame.clone());
                let frag_list = &self
                    .fragmented
                    .get(frame.fragment_info.unwrap().fragment_id);

                if frag_list.is_some() {
                    let mut list = frag_list.clone().unwrap();
                    let pk = list.reassemble_frame();
                    if pk.is_some() {
                        self.handle_full_frame(&mut pk.unwrap());
                        self.fragmented
                            .remove(frame.fragment_info.unwrap().fragment_id.into());
                    }
                }
                continue;
            } else {
                self.handle_full_frame(frame);
            }
        }
    }

    /// Handles the full frame from the client.
    fn handle_full_frame(&mut self, frame: &mut Frame) {
        // todo Check if the frames should be recieved, if not purge them
        // todo EG: add implementation for ordering and sequenced frames!
        let mut body_stream = Cursor::new(frame.body.clone());
        let online_packet = OnlinePackets::from_byte(body_stream.read_u8().unwrap());

        if online_packet == OnlinePackets::GamePacket {
            // self.recv.as_ref()(self, &mut body_stream.get_mut());
            // we don't really care what happens to game packet, so emit it.
            self.event_dispatch.push_back(RakEvent::GamePacket(
                self.address_token.clone(),
                frame.body.clone(),
            ));
        } else {
            if let Ok(response) = handle_online(self, online_packet.clone(), &mut frame.body) {
                if response.len() != 0 {
                    let mut new_framepk = FramePacket::new();
                    let mut new_frame = Frame::init();

                    new_frame.body = response;
                    new_frame.reliability = Reliability::new(ReliabilityFlag::Unreliable);
                    new_framepk.frames.push(new_frame);
                    new_framepk.seq = self.send_seq.into();

                    self.send_stream(new_framepk.fparse(), true);
                    self.send_seq = self.send_seq + 1;
                }
            }
        }
    }

    pub fn next_send_seq(&mut self) -> u32 {
        let old = self.send_seq.clone();
        self.send_seq += 1;
        old
    }

    /// Called when RakNet is ready to "tick" this client.
    /// Each "tick" the following things are done:
    ///
    /// - Send all **Ack** and **Nack** queues to the client.
    ///
    /// - Fragments everything in the `send_queue_large` queue,
    ///   and then appends all of these "buffers" or "Streams"
    ///   to be sent by raknet on the next iteration.
    pub fn do_tick(&mut self) {
        if self.state == ConnectionState::Offline {
            return;
        }

        if self.recv_time.elapsed().unwrap().as_secs() >= 10 {
            self.state = ConnectionState::Offline;
            self.event_dispatch.push_back(RakEvent::Disconnect(
                tokenize_addr(self.address),
                "Time Out".to_owned(),
            ));
            return;
        }

        if self.recv_time.elapsed().unwrap().as_secs() >= 5 && self.state.is_reliable() {
            self.state = ConnectionState::TimingOut;
            return;
        }

        if self.state.is_reliable() {
            if !self.ack.is_empty() {
                let respond_with = self.ack.make_ack();
                self.send_stream(respond_with.fparse(), true);
            }

            if !self.nack.is_empty() {
                let respond_with = self.nack.make_nack();
                self.send_stream(respond_with.fparse(), true);
            }
        }

        let mut current_frames: Vec<FragmentList> = Vec::new();
        let safe_size = self.mtu_size - 15;

        // Make seperate buffers from the large queue based on the MTUSize saving
        // space for frame packet header
        for part in self.send_queue_large.iter_mut() {
            current_frames.push(FragmentList::from(part, safe_size.into()));
        }

		self.send_queue_large.clear();

        for safely_sized in current_frames.iter_mut() {
            let packets = safely_sized.assemble(safe_size as i16, self.fragment_id);

            if packets.is_some() {
                for pk in packets.unwrap() {
					if cfg!(any(test, feature = "dbg")) {
						log_online(format!("[{}] Sent packet: {:#?}\n", self.address_token, &pk));
					}
                    self.send_stream(pk.fparse(), true);
                }

                self.fragment_id += 1;

                if self.fragment_id == 65534 {
                    self.fragment_id = 0;
                }
            }
        }
    }

    pub fn get_motd(&self) -> Motd {
        self.motd.clone()
    }
}
