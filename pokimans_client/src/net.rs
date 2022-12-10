use std::sync::Arc;
use bevy::prelude::Res;
use tokio::sync::Mutex;
use tokio::net::TcpStream;
use pokimans_common::tokio::Tokio;


pub fn setup_client(tk: Res<Tokio>) {
    println!("Connecting to pokimans network");
    tk.runtime.spawn(async {
	let stream = Arc::new(Mutex::new(
	    TcpStream::connect("127.0.0.1:8080").await.unwrap()
	));

	let mut buf = vec![0; 1024];

	let stream_clone1 = Arc::clone(&stream);
	let receive_messages = async move {
	    loop {}
	};

	let stream_clone2 = Arc::clone(&stream);
	let send_messages = async move {
	    loop {}
	};
	
	tokio::join!(receive_messages, send_messages);
    });
}
    

 
