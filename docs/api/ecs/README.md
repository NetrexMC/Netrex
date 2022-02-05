# Entity Component System
Netrex uses an entity component system to manage entities within worlds. This allows for a very flexible and powerful entity system to be implemented. This page will explain the concept of
the **Entity Component System** and how it's used within Netrex. If you're looking for how Entities are implemented within a world, visit the [Worlds](/worlds/README.md) documentation.

## How it works?
Netrex takes a huge inspiration from the [Entity Component System](https://en.wikipedia.org/wiki/Entity_component_system) design and uses it to create a powerful, dynamic API.

The main concept with a **Entity Component Sytem** is that Entities are composed of components; each component being a implementation of the `Component` trait, and as such, does not need
to be implemented for every entity. This allows all entities to be dynamic and modified at any point. 

For example, imagine we want a `Player` entity to have infinite health because they are now in spectator mode; We can simple just register an `InfiniteHealth` component to the player, and now that player will have inifinite health until we remove that component.
One question that arises is how do we differentiate a `Player` entity from a `Mob` or something? Simple! We check if the entity has a `Player` component.

This design allows us to be flexible in that not every entity needs to share the same components, remaining implicit while still being explicit.

## Real Example
Another example could be, if we want to track the position of an entity within the world, we need to make a `Position` component, as well as implementing a `PositionSystem` on the world.
To implement this in code, we first need to create our new `Position` struct and derive a `Component`, as shown below:
```rust
use netrex::cs::Entity;
use netrex::cs::Component;

#[derive(Component)]
pub struct Position {
	pub x: f32,
	pub y: f32,
	pub z: f32
};

impl Position {
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Self {
			x, y, z
		}
	}

	pub fn sub(&self, other: &Self) -> Self {
		Self {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z
		}
	}
}
```
Now that we have the `Position` as a registered component, we need to give it functionality.
Currently, our component does nothing, let's implement a system on-top of it, we'll be doing this by implementing the `EntitySystem` trait onto our `PositionSystem` struct.:

?> Note: You do not need to create an entire `PositionSystem` struct for this implementation, you can implement it on `Position` itself.

```rust
use netrex::cs::EntitySystem;

#[derive(auto_init)]
pub struct PositionSystem;

impl EntitySystem for PositionSystem {
	type In = Position;
	type Out = ();

	fn update_entity(&mut self, mut to: Self::In, entity: &mut Entity) -> Self::Out {
		// this will only be called if the entity even has the Position Component.
		// in a real scenario, the manager would be an instance of the world.
		if let Ok(pos) = entity.get_component_mut::<Position>() {
			pos.x += to.x;
			pos.y += to.y;
			pos.z += to.z;
		}
	}
}
```

Now that we have our `PositionSystem` with our components, we can add it to the world!
```rust
use netrex::world::World;
use netrex::world::WorldManager;

fn add_position_system(world: &mut World) {
	world.add_system(PositionSystem::new());
}
```
Now that we have the system registered, we can update an entity with any event, however for this example we will move all entities up by 1 unit.
```rust
use netrex::world::World;
use netrex::world::WorldManager;

fn move_once() {
	if let Ok(world) = WorldManager::get_world_mut("hello") {
		// get the entity system.
		let system = world.get_system::<PositionSystem>().unwrap();

		for entity in world.get_entities().iter_mut() {
			let mut to = Position::new(1, 1, 1);
			system.update_entity(to, &mut entity);
		}
	}
}
```