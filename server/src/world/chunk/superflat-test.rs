impl WorldGenerator for SuperFlatGenerator {
	async fn generate_chunk(&mut self, sub_x: i32, sub_z: i32, world: &'static mut dyn World) -> Chunk {
		let mut chunk = SuperFlatChunk::new(sub_x, sub_z);
		chunk.fill(5, |x, y, z| {
			if y == 0 {
				Block::Bedrock
			} else if y <= 4 {
				Block::Dirt
			} else if y == 5 {
				Block::Grass
			} else {
				Block::Air
			}
		});
		chunk.set_biome(Biomes::Plains);
		chunk.populator["grass"] = Box::new(|_, _, _, _| {});
		return chunk;
	}
}