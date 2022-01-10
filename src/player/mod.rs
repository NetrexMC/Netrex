use crate::network::session::Session;

pub mod skin;

// use session::PlayerSession;

pub struct Player {
    pub(crate) session: Session,
}

impl Player {
	pub fn new(session: Session) -> Self {
		Player { session }
	}
}
