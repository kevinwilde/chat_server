use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;

use chatmap::{ChatMap, ClientInfo};
use message::Message;

pub fn create_client(stream: TcpStream, sender_to_router: Sender<Message>, chat_map: &Arc<Mutex<ChatMap>>) {
    println!("New client");
    let mut stream = stream;
    stream.write(&"Welcome to Smazy\n".to_string().into_bytes()).unwrap();
    stream.write(&"Please enter a username:\n".to_string().into_bytes()).unwrap();
    stream.flush().unwrap();
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut username = "".to_string();
    match reader.read_line(&mut username) {
        Ok(n) => {
            let username = username.trim().to_string();
            let (sender_to_client, receiver_from_router) = channel();
            {
                let mut guard = chat_map.lock().unwrap();
                if n > 0 && !guard.contains_key(&username) {
                    let client_info = ClientInfo{partner: None, sender_to_client: sender_to_client};
                    guard.insert(username.to_string(), client_info);
                    println!("New user: {}", username.to_string());
                }
            }
            let partner = choose_chat_partner(stream.try_clone().unwrap(), username.to_string(), chat_map);
            chat(stream, username, partner, sender_to_router, receiver_from_router);
        },
        Err(e) => println!("{}", e)
    }
}

fn choose_chat_partner(stream: TcpStream, username: String, chat_map: &Arc<Mutex<ChatMap>>) -> String {
    let mut stream = stream;
    
    stream.write(&"Here are the users available to chat:\n".to_string().into_bytes()).unwrap();
    {
        let guard = chat_map.lock().unwrap();
        for (name, client_info) in guard.iter() {
            if name.as_str() != username.as_str() && client_info.partner == None {
                stream.write(&name.to_string().into_bytes()).unwrap();
                stream.write(&"\n".to_string().into_bytes()).unwrap();
            }
        }
    }
    
    stream.write(&"Select who you want to chat with:\n".to_string().into_bytes()).unwrap();
    
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut partner = "".to_string();
    
    match reader.read_line(&mut partner) {
        Ok(_) => {
            partner = partner.trim().to_string();

            if !try_select_partner(chat_map, username.to_string(), partner.to_string()) {
                return choose_chat_partner(stream, username, chat_map);
            }            
        }
        Err(e) => println!("{}", e)
    }
    
    partner
}

fn try_select_partner(chat_map: &Arc<Mutex<ChatMap>>, username: String, partner: String) -> bool {
    let success: bool;
    {
        let mut guard = chat_map.lock().unwrap();
        
        match guard.get_mut(&partner) {
            Some(clientinfo) => {
                if partner.as_str() != username.as_str() 
                    && (clientinfo.partner == None 
                        || clientinfo.partner == Some(username.to_string())) {
                    clientinfo.partner = Some(username.to_string());
                    success = true;
                } else {
                    success = false;
                }
            },
            None => success = false
        }
    }

    if success {
        let mut guard = chat_map.lock().unwrap();
        println!("Assigning partner {} to {}", partner, username);
        guard.get_mut(&username).unwrap().partner = Some(partner.to_string());
    }
    success
}

// TODO: Receiving a message while in the middle of typing a message
//       inserts received message into middle of your message
fn chat(stream: TcpStream, username: String, partner: String, sender_to_router: Sender<Message>, receiver_from_router: Receiver<Message>) {
    // Send messages
    {
        let username = username.clone();
        let reader = BufReader::new(stream.try_clone().unwrap());
        thread::spawn(move|| {
            let mut lines = reader.lines(); 
            while let Some(Ok(line)) = lines.next() {
                println!("{}",line);
                let msg = Message::new("Date".to_string(), username.to_string(), partner.to_string(), line.to_string());
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
                        stream.write(&msg.from().to_string().into_bytes()).unwrap();
                        stream.write(&": ".to_string().into_bytes()).unwrap();
                        stream.write(&msg.content().to_string().into_bytes()).unwrap();
                        stream.write(&"\n".to_string().into_bytes()).unwrap();
                    }
                    Err(TryRecvError::Empty) => continue,
                    Err(TryRecvError::Disconnected) => panic!("User {} disconnected from router", &username)
                }
            }
        });
    }
}