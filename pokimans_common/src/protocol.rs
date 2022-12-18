use serde::{Serialize, Deserialize};

// Message that originate from client
#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    PlayerJoin,
    PlayerMove {
	target: (i32, i32),
    },
}

// Messages originating from server
#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    PlayerJoin {
	id: u32, // Player entity id ON SERVER. Will be different on client most likely.
    },
    PlayerMove {
	id: u32,
	target: (i32, i32),
    },
}
