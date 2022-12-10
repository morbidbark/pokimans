use bevy::prelude::*;
use crate::utils::Position;

pub const CHUNK_SIZE: i32 = 16;

#[derive(Debug)]
pub struct Cell {
    pub position: Position,
    pub traversible: bool,
}

fn make_square_chunk() -> Vec<Cell> {
    let mut chunk: Vec<Cell> = vec![];
    for y in 0..CHUNK_SIZE as i32 {
	for x in 0..CHUNK_SIZE as i32 {
	    let position = Position { x, y };
	    let mut traversible = true;
	    if x==0 || y==0 || x==CHUNK_SIZE-1 || y == CHUNK_SIZE-1 {
		traversible = false;
	    }
	    chunk.push(Cell { position, traversible });
	}
    }
    chunk
}

#[derive(Resource)]
pub struct Map {
    pub chunks: Vec<Vec<Cell>>,
}
pub fn setup_map(mut commands: Commands) {
    commands.insert_resource(Map { chunks: vec![make_square_chunk()] });
}
