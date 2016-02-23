// Christopher Chen
// Alex Cohen
// Thornton Uhl
// Kevin Wilde

use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::thread;

use message::Message;

mod client;
mod message;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let usernames = Arc::new(Mutex::new(HashMap::new()));
    let (sender_to_router, receiver_from_clients) = channel();

    //Router thread
    {
        let usernames = usernames.clone();
        thread::spawn(move|| {
            loop {
                // TODO: use recv or try_recv here?
                let msg: Message = receiver_from_clients.recv().unwrap();
                println!("Router received message date {}, from {}, to {}, content {}", 
                    msg.date(), msg.from(), msg.to(), msg.content());

                let guard = usernames.lock().unwrap();
                if let Some(sender) = guard.get(msg.to()) {
                    let sender: &Sender<Message> = sender;
                    sender.send(msg).unwrap();
                } else {
                    println!("{} does not exist in hashmap", msg.to());
                }
            }
        });
    }

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let usernames = usernames.clone();
                let sender_to_router = sender_to_router.clone();
                thread::spawn(move|| {
                    // connection succeeded
                    client::create_client(stream, sender_to_router, &usernames);
                });
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    // close the socket server
    drop(listener);
}

