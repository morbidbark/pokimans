use std::collections::HashMap;

use bevy::prelude::*;


pub const CHUNK_SIZE: i32 = 16;

#[derive(Debug)]
pub struct Cell {
    pub traversible: bool,
}

fn make_square_chunk() -> HashMap<(i32, i32), Cell> {
    let mut chunk = HashMap::new();
    for y in 0..CHUNK_SIZE as i32 {
	for x in 0..CHUNK_SIZE as i32 {
	    let mut traversible = true;
	    if x == 0 || y == 0 || x == CHUNK_SIZE - 1 || y == CHUNK_SIZE - 1 {
		traversible = false;
	    }
	    chunk.insert(
		(x, y),
		Cell { traversible },
	    );
	}
    }
    chunk
}

#[derive(Resource)]
pub struct Map {
    pub chunks: Vec<HashMap<(i32, i32), Cell>>,
}

pub fn setup_map(mut commands: Commands) {
    commands.insert_resource(Map { chunks: vec![make_square_chunk()] });
}
