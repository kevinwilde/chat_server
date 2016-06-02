// Christopher Chen
// Alex Cohen
// Thornton Uhl
// Kevin Wilde

use std::net::TcpListener;
use std::thread;

use server::Server;

mod message;
mod roommap;
mod server;
mod usermap;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
                                .expect("Error binding listener");
    
    let server = Server::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    client::create_client(stream, &server);
                });
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    drop(listener);
}
