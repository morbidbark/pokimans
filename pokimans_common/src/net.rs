use bevy::prelude::*;
use tokio::sync::mpsc;
use crate::protocol::Message;

#[derive(Resource)]
pub struct Network {
    pub rx: mpsc::Receiver<Message>,
    pub tx: mpsc::Sender<Message>,
}

