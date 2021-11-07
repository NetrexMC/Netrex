use rakrs::{RakNetServer, conn::Connection};

pub struct PlayerSession<'a> {
	connection: &'a mut Connection
}