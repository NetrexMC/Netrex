use rakrs::{conn::Connection, RakNetServer};

pub struct PlayerSession<'a> {
    connection: &'a mut Connection,
}
