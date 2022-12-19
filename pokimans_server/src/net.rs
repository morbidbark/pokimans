use std::sync::Arc;
use std::collections::HashMap;
use bevy::ecs::entity;
use bevy::prelude::*;
use pokimans_common::movement::MovementControllerBundle;
use tokio::sync::{Mutex, mpsc};
use tokio::net::{TcpListener, TcpStream};

use pokimans_common::{
    movement::{MoveEvent, MovementController},
    protocol::{ClientMessage, ServerMessage},
    tokio::Tokio,
};

#[derive(Resource)]
pub struct Network {
    pub rx: mpsc::Receiver<(String, ClientMessage)>,
    pub tx: mpsc::Sender<ServerMessage>,
}

#[derive(Resource)]
pub struct PlayerMap(HashMap<String, Entity>);

pub fn setup_server(mut commands: Commands, tk: Res<Tokio>) {
    println!("Establishing pokimans network");

    let (rx_in, rx) = mpsc::channel::<(String, ClientMessage)>(64);
    let (tx, mut tx_out) = mpsc::channel::<ServerMessage>(64);

    tk.runtime.spawn(async {
	let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Could not bind to socket");

	let streams: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(vec![]));

	let streams_clone1 = Arc::clone(&streams);
	let accept_connections = async move {
	    loop {
		let (stream, addr) = listener.accept().await.unwrap();
		let mut lock = streams_clone1.lock().await;
		lock.push(stream);
		println!("Connection establishing with {:?}", addr);
	    }
	};

	let streams_clone2 = Arc::clone(&streams);
	let read_from_streams = async move {
	    loop {
		let lock = streams_clone2.lock().await;
		for stream in lock.iter() {
		    let mut buf = [0; 1024];
		    match stream.try_read(&mut buf) {
			Ok(0) => continue,
			Ok(_) => {
			    let string = String::from_utf8_lossy(&buf); 
			    let message: ClientMessage = ron::from_str(string.trim_matches(char::from(0))).unwrap();
			    rx_in.send((stream.peer_addr().unwrap().to_string(), message)).await.unwrap();
			},
			Err(_) => continue,
		    }
		}
	    }
	};

	tokio::join!(accept_connections, read_from_streams);
    });

    let network = Network { rx, tx };
    commands.insert_resource(network);

    let player_map: HashMap<String, Entity> = HashMap::new();
    commands.insert_resource(PlayerMap(player_map));
}

pub fn handle_request_player_entity() {}

pub fn handle_player_move() {}

pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
	app
	    .add_system(handle_request_player_entity)
	    .add_system(handle_player_move);
    }
}
