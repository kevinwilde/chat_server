use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

use server::Server;

mod client;
mod command;
mod message;
mod roommap;
mod server;
mod usermap;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
                               .expect("Error binding listener");
    
    let server = Arc::new(Mutex::new(Server::new()));

    for stream in listener.incoming() {
        match stream {
            
            Ok(stream) => {
                let server = server.clone();
                thread::spawn(move|| {
                    client::run(stream, &server);
                });
            }
            
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    drop(listener);
}
