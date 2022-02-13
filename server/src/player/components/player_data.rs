#[derive(readonly::make)]
pub struct PlayerData {
	/// The Name of the player when they logged in.
	#[readonly]
	pub name: String,
	/// The Name or (tag) of the player internally. This is **NOT** the name used to identify a player.
	/// To identify a player, use the `name` field.
	pub display_name: String,
	/// The Player's Skin.
	pub skin: Skin,
	/// The Player's UUID.
	/// This is the unique identifier for the player, (this is not saved)
	#[readonly]
	pub uuid: String,
	/// The Player's XUID.
	/// The ID assigned by Mojang to the player.
	/// If the player is not logged in with XBL, this will be a string containing `NO_AUTH`.
	#[readonly]
	pub xuid: String
}

impl Component for PlayerData {

}