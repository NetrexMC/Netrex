pub mod util;
use binary_utils::BinaryStream;

pub trait ServerBound {
	type Packet;

	/// Converts a `BinaryStream` into a Server bound packet.
	fn from(stream: BinaryStream) -> Self::Packet;
}

pub trait ClientBound {
	/// Converts a Packet to a `BinaryStream` which is later sent to RakNet.
	fn to(&mut self) -> BinaryStream;
}
