// Christopher Chen
// Alex Cohen
// Thornton Uhl
// Kevin Wilde

use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

use chatmap::ChatMap;

mod chatmap;
mod client;
mod command;
mod message;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Error binding listener");
    let chat_map: Arc<Mutex<ChatMap>> = Arc::new(Mutex::new(ChatMap::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let chat_map = chat_map.clone();
                thread::spawn(move|| {
                    client::create_client(stream, &chat_map);
                });
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    drop(listener);
}

