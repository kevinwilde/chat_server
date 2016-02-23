// Christopher Chen
// Alex Cohen
// Thornton Uhl
// Kevin Wilde

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let usernames = Arc::new(Mutex::new(HashMap::new()));
    let (sender_to_router, receiver_from_clients) = channel();

    //Router thread
    thread::spawn(move|| {
        loop {
            let message = receiver_from_clients.recv().unwrap();
            println!("Router received {}", message);
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
                    create_client(stream, &usernames, sender_to_router);
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

fn create_client(stream: TcpStream, usernames: &Arc<Mutex<HashMap<String, Sender<String>>>>, sender_to_router: Sender<String>) {
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
                sender_to_router.send("Hi".to_string()).unwrap();
                let message = receiver_from_router.recv().unwrap();
                println!("User received {}", message);
                println!("end of loop");
            }
        },
        Err(e) => println!("{}", e)
    }
}
