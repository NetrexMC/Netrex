// Login Types

#[derive(Clone, Debug)]
pub struct PlayerLoginData {
    /// The Name of the player
    pub name: String,
    /// The UUID of the player
    pub id: String,
    /// The device the player is playing on
    pub os_id: String,
    /// The xbox user id
    pub xuid: String,
}

pub enum OperatingSystem {
	Windows,
	Linux,
	MacOS,
	Android,
	iOS,
	XboxOne,
	PS4,
	Switch,
	Unknown,
}