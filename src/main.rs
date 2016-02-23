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
            let username = username.trim().to_string();
            let (sender_to_client, receiver_from_router) = channel();
            {
                let mut guard = usernames.lock().unwrap();
                if n > 0 && !guard.contains_key(&username) {
                    println!("Inserting {}", &username);
                    guard.insert(username.to_string(), sender_to_client);
                    println!("Username is {}", username.to_string());
                }
            }

            // TODO: show list of available to users and choose who to chat with
            // Include bool in hashmap for available? 
            // Or Option<String> that says who you are chatting with (None if not chatting)

            // Send messages
            {
                let username = username.clone();
                thread::spawn(move|| {
                    let mut lines = reader.lines(); 
                    while let Some(Ok(line)) = lines.next() {
                        println!("{}",line);
                        let msg = Message::new("Date".to_string(), username.to_string(), "b".to_string(), line.to_string());
                        sender_to_router.send(msg).unwrap();
                    }                
                });
            }


            // Receive Messages
            {
                let username = username.clone();
                thread::spawn(move|| {
                    loop {
                        match receiver_from_router.try_recv() {
                            Ok(msg) => {
                                println!("User {} received message: {}", &username, msg.content());
                                //stream.write(&msg.content().into_bytes());
                            }
                            Err(TryRecvError::Empty) => continue,
                            Err(TryRecvError::Disconnected) => panic!("User {} disconnected from router", &username)
                        }
                    }
                });
            }
        },
        Err(e) => println!("{}", e)
    }
}
