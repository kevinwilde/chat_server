// Christopher Chen
// Alex Cohen
// Thornton Uhl
// Kevin Wilde

use std::collections::HashSet;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let usernames = Arc::new(Mutex::new(HashSet::new()));

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let usernames = usernames.clone();
                thread::spawn(move|| {
                    // connection succeeded
                    create_client(stream, &usernames);
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

fn create_client(stream: TcpStream, usernames: &Arc<Mutex<HashSet<String>>>) {
    println!("New client");
    let mut stream = stream;
    stream.write(&"Welcome to Smazy\n".to_string().into_bytes()).unwrap();
    stream.write(&"Please enter a username:\n".to_string().into_bytes()).unwrap();
    stream.flush().unwrap();
    let mut reader = BufReader::new(stream);
    let mut username = "".to_string();
    match reader.read_line(&mut username) {
        Ok(n) => {
            let mut guard = usernames.lock().unwrap();
            if n > 0 && !guard.contains(&username) {
                guard.insert(username.to_string());
                println!("Username is {}", username.to_string());
            }
        },
        Err(e) => println!("{}", e)
    }
}
