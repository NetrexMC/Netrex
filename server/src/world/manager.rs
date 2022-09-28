lazy_static! {
	pub static ref WORLD_MANAGER: Mutex<WorldManager> = Mutex::new(WorldManager::new());
}

// This struct is a master struct,
// it controls the ticking, and the schedulers for each world.
pub struct WorldManager {

}