use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    create_client(stream);
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

fn create_client(stream: TcpStream) {
    println!("New client");
    let mut stream = stream;
    stream.write(&"Welcome to Smazy".to_string().into_bytes()).unwrap();
    stream.write(&"Please enter a username".to_string().into_bytes()).unwrap();
    stream.flush();
    let mut username = "".to_string();
    stream.read_to_string(&mut username).unwrap();
    println!("Username is {}", username);
}