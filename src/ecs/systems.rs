use std::collections::HashMap;

pub type SystemId = u16;

/// A generic system, these systems don't need to be executable.
/// However they can be, an example of a system using this would be a `NetworkSystem`.
pub trait System {
	/// Gets the ID of the system.
	fn get_id(&self) -> SystemId;
	/// Gets the name of the system.
	fn get_name(&self) -> &'static str;

	/// This function is used only once, when the system is added to the world.
	/// You should check for this in your implementation.
	fn register(&mut self, id: SystemId);

	// / Queries the System for a component.
	// / This is not used yet.
	// fn query(&self, component: &Component) -> bool;
}