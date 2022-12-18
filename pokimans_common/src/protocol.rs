use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    ChatMessage {
	message: String
    },
    PlayerMove {
	target: (i32, i32)
    },
}
