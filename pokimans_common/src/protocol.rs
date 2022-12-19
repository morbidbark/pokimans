use serde::{Serialize, Deserialize};
use bevy::prelude::*;

// Message that originate from client
#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    RequestPlayerEntity,
    PlayerJoin,
    PlayerMove {
	direction: Vec2,
    },
}

// Messages originating from server
#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    PlayerEntitySpawned {
	id: Entity,
    },
    PlayerJoin {
	id: Entity, // Player entity id ON SERVER. Will be different on client most likely.
    },
    PlayerMove {
	id: Entity,
	direction: Vec2,
    },
}
