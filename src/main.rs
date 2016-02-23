// Christopher Chen
// Alex Cohen
// Thornton Uhl
// Kevin Wilde

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::thread;

use message::Message;

mod message;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let usernames = Arc::new(Mutex::new(HashMap::new()));
    let (sender_to_router, receiver_from_clients) = channel();

    //Router thread
    thread::spawn(move|| {
        loop {
            // TODO: use recv or try_recv here?
            let msg: Message = receiver_from_clients.recv().unwrap();
            println!("Router received message date {}, from {}, to {}, content {}", 
                msg.date, msg.from, msg.to, msg.content);
            // TODO: lookup recipient in hashmap and forward msg.content
        }
    });

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let usernames = usernames.clone();
                let sender_to_router = sender_to_router.clone();
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream, sender_to_router, &usernames);
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

fn handle_client(stream: TcpStream, sender_to_router: Sender<Message>, usernames: &Arc<Mutex<HashMap<String, Sender<Message>>>>) {
    println!("New client");
    let mut stream = stream;
    stream.write(&"Welcome to Smazy\n".to_string().into_bytes()).unwrap();
    stream.write(&"Please enter a username:\n".to_string().into_bytes()).unwrap();
    stream.flush().unwrap();
    let mut reader = BufReader::new(stream);
    let mut username = "".to_string();
    match reader.read_line(&mut username) {
        Ok(n) => {
            let (sender_to_client, receiver_from_router) = channel();
            let mut guard = usernames.lock().unwrap();
            if n > 0 && !guard.contains_key(&username) {
                guard.insert(username.to_string(), sender_to_client);
                println!("Username is {}", username.to_string());
            }

            loop {
                let msg = message::Message{
                    date: "Date".to_string(), 
                    from: username.to_string(),
                    to: "b".to_string(),
                    content: "Hi".to_string()
                };
                sender_to_router.send(msg).unwrap();
                match receiver_from_router.try_recv() {
                    Ok(msg) => println!("User {} received message: {}", &username, msg.content),
                    Err(TryRecvError::Empty) => continue,
                    Err(TryRecvError::Disconnected) => panic!("User {} disconnected from router", &username)
                }
            }
        },
        Err(e) => println!("{}", e)
    }
}
