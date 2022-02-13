use super::component::Component;
use std::{collections::HashMap, any::TypeId};


/// Entity is a wrapper around a unique identifier.
pub struct Entity {
	/// The unique ID for this entity.
	pub id: u32,
	/// Components of the entity.
	components: HashMap<TypeId, Box<dyn Component>>,
}

impl Entity {
	pub fn new(id: u32) -> Self {
		Self {
			id,
			components: HashMap::new(),
		}
	}

	/// Initializes an empty entity with the ID of 0.
	pub fn init() -> Self {
		Self {
			id: 0,
			components: HashMap::new(),
		}
	}

	pub fn add_component<T: Component>(&mut self, component: T) {
		let id = TypeId::of::<T>();
		let mut component = Box::new(component);
		component.set_id(self.id);
		self.components.insert(id, component);
	}

	pub fn get_component<T: Component>(&self) -> Option<&T> {
		None
	}
}