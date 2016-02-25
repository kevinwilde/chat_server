// Christopher Chen
// Alex Cohen
// Thornton Uhl
// Kevin Wilde

use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;

use chatmap::ChatMap;
use message::Message;

mod client;
mod chatmap;
mod message;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let chat_map: Arc<Mutex<ChatMap>> = Arc::new(Mutex::new(ChatMap::new()));
    let (sender_to_router, receiver_from_clients) = channel();

    //Router thread
    {
        let chat_map = chat_map.clone();
        thread::spawn(move|| {
            loop {
                let msg: Message = receiver_from_clients.recv().unwrap();
                println!("Router received message date {}, from {}, to {}, content {}", 
                    msg.date(), msg.from(), msg.to(), msg.content());

                let guard = chat_map.lock().unwrap();

                if let Some(client_info) = guard.get(msg.to()) {
                    if let Some(ref p) = client_info.partner {
                        assert_eq!(&p[..], &msg.from()[..]);
                        client_info.sender_to_client.send(msg).unwrap();
                    }
                } else {
                    println!("{} does not exist in hashmap", msg.to());
                }
            }
        });
    }

    // Clients
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let chat_map = chat_map.clone();
                let sender_to_router = sender_to_router.clone();
                thread::spawn(move|| {
                    client::create_client(stream, sender_to_router, &chat_map);
                });
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    drop(listener);
}

