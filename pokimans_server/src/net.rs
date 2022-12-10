use std::sync::Arc;
use bevy::prelude::Res;
use tokio::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};

use pokimans_common::tokio::Tokio;

pub fn setup_server(tk: Res<Tokio>) {
    println!("Establishing pokimans network");

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
			Ok(_nbytes) => println!("{:?} says {}", stream.peer_addr(), String::from_utf8_lossy(&buf)),
			Err(_) => continue,
		    }
		}
	    }
	};

	tokio::join!(accept_connections, read_from_streams);
    });
}
