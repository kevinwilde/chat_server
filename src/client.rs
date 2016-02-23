use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;

use message::Message;

pub fn create_client(stream: TcpStream, sender_to_router: Sender<Message>, usernames: &Arc<Mutex<HashMap<String, Sender<Message>>>>) {
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
            chat(reader.into_inner(), username, sender_to_router, receiver_from_router);
        },
        Err(e) => println!("{}", e)
    }
}

fn chat(stream: TcpStream, username: String, sender_to_router: Sender<Message>, receiver_from_router: Receiver<Message>) {
    // TODO: show list of available to users and choose who to chat with
    // Include bool in hashmap for available? 
    // Or Option<String> that says who you are chatting with (None if not chatting)

    // Send messages
    {
        let username = username.clone();
        let reader = BufReader::new(stream.try_clone().unwrap());
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
        let mut stream = stream;
        thread::spawn(move|| {
            loop {
                match receiver_from_router.try_recv() {
                    Ok(msg) => {
                        println!("User {} received message: {}", &username, msg.content());
                        stream.write(&msg.content().to_string().into_bytes()).unwrap();
                    }
                    Err(TryRecvError::Empty) => continue,
                    Err(TryRecvError::Disconnected) => panic!("User {} disconnected from router", &username)
                }
            }
        });
    }
}