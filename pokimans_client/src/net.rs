use std::io;
use std::sync::Arc;
use bevy::prelude::{Res, Commands};
use tokio::sync::{Mutex, mpsc};
use tokio::net::TcpStream;
use pokimans_common::tokio::Tokio;
use pokimans_common::protocol::Message;
use pokimans_common::net::Network;


pub fn setup_client(mut commands: Commands, tk: Res<Tokio>) {
    println!("Connecting to pokimans network");

    let (rx_in, rx) = mpsc::channel::<Message>(64);
    let (tx, mut tx_out) = mpsc::channel::<Message>(64);

    tk.runtime.spawn(async {
	let stream = Arc::new(Mutex::new(
	    TcpStream::connect("127.0.0.1:8080").await.unwrap()
	));

	let stream_clone1 = Arc::clone(&stream);
	let receive_messages = async move {
	    let mut buf = vec![0; 1024];
	    loop {
		let lock = stream_clone1.lock().await;
		match lock.try_read(&mut buf) {
		    Ok(0) => continue ,
		    Ok(_) => {
			let string = String::from_utf8_lossy(&buf); 
			println!("Received message {}", &string);
			let message: Message = ron::from_str(string.trim_matches(char::from(0))).unwrap();
			rx_in.send(message).await.unwrap();
		    }
		    Err(e) => {
			println!("{}", e);
		    }
		}
	    }
	};

	let stream_clone2 = Arc::clone(&stream);
	let send_messages = async move {
	    loop {
		let lock = stream_clone2.lock().await;
		while let Some(message) = tx_out.recv().await {
		    let data = ron::to_string(&message).unwrap();
		    match lock.try_write(data.as_bytes()) {
			Ok(_) => println!("Sent: {:?}", message),
			Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
			Err(e) => println!("{}", e),
		    }
		}
	    }
	};

	tokio::join!(receive_messages, send_messages);
    });
    
    let network = Network { rx, tx };
    commands.insert_resource(network);	
}
    

 
