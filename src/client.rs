use std::io::{BufRead, BufReader, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

// use command::Command;
use message::Message;
use server::Server;

// use command::parse_command;

pub fn run(stream: TcpStream, server: &Arc<Mutex<Server>>) {
    println!("New client");    
    Server::welcome_user(clone_stream(&stream));
    let mut client = Client::new(clone_stream(&stream));
    choose_username(&mut client, server);
    Server::display_instructions(clone_stream(&stream));
    chat(client, server);
}

pub struct Client {
    username: String,
    stream: TcpStream,
    sndr: Sender<String>,
    rcvr: Receiver<String>,
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        let (sndr, rcvr) = channel();
        Client {
            username: "".to_string(),
            stream: stream,
            sndr: sndr,
            rcvr: rcvr,
        }
    }
}

fn choose_username(client: &mut Client, server: &Arc<Mutex<Server>>) {
    let mut reader = BufReader::new(clone_stream(&client.stream));
    let mut username = "".to_string();

    loop {
        match reader.read_line(&mut username) {
            Ok(_) => {
                username = username.trim().to_string().to_lowercase();

                if server.lock().unwrap().add_user(username.to_string()) {
                    client.username = username.to_string();
                    break
                }
                else {
                    let mut stream = clone_stream(&client.stream);
                    let invalid_msg = "Invalid username. Please try again.\n".to_string();
                    stream.write(&invalid_msg.into_bytes())
                          .expect("Error writing to stream");
                    username = "".to_string();
                }
            },
            Err(e) => println!("{}", e)
        }
    }
}

fn choose_chatroom(client: &Client, server: &Arc<Mutex<Server>>) -> usize {
    // Display rooms and give option to create new room
    let mut stream = clone_stream(&client.stream);
    let msg = "Available rooms:\n".to_string();
    stream.write(&msg.into_bytes()).expect("Error writing to stream");

    server.lock().unwrap().display_rooms(clone_stream(&stream));

    let msg = "\nEnter the room number of the room you wish to join, \
                or type \"new\".\n".to_string();
    stream.write(&msg.into_bytes()).expect("Error writing to stream");

    // If create new room, call server.create_room
    let mut reader = BufReader::new(clone_stream(&stream));
    let mut choice = "".to_string();
    loop {
        match reader.read_line(&mut choice) {
            Ok(_) => {
                choice = choice.trim().to_string().to_lowercase();

                if choice == "new".to_string() {
                    let msg = "Enter the name of your room: ".to_string();
                    stream.write(&msg.into_bytes()).expect("Error writing to stream");
                    let mut room_name = "".to_string();
                    if let Ok(_) = reader.read_line(&mut room_name) {
                        let mut server = server.lock().unwrap();
                        let room_num = server.create_room(room_name.trim().to_string());
                        server.join_room(room_num, client.username.to_string(),
                                         client.sndr.clone());
                        return room_num;
                    }
                }
                else if let Ok(room_num) = choice.parse() {
                    if server.lock().unwrap()
                            .join_room(room_num, client.username.to_string(),
                                       client.sndr.clone()) {
                        return room_num;
                    }
                }
                let msg = "Try again\n".to_string();
                choice = "".to_string();
                stream.write(&msg.into_bytes()).expect("Error writing to stream");
            },
            Err(e) => println!("{}", e)
        }
    }
}

fn chat(client: Client, server: &Arc<Mutex<Server>>) {

    let room_num = choose_chatroom(&client, server);

    // Send messages
    {
        let username = client.username.to_string();
        let reader = BufReader::new(clone_stream(&client.stream));
        let server = server.clone();

        thread::spawn(move|| {
            let mut lines = reader.lines(); 
            while let Some(Ok(line)) = lines.next() {
                if line.len() > 0 {
                    let msg = Message::new(username.to_string(),
                                           room_num,
                                           line.to_string());
                    server.lock().unwrap().send_message(msg);
                }
            }
        });
    }

    // Receive Messages
    {
        thread::spawn(move|| {
            let mut stream = clone_stream(&client.stream);
            loop {
                match client.rcvr.recv() {
                    Ok(msg) => {
                        stream.write(&msg.into_bytes())
                              .expect("Error writing to stream");
                    },

                    Err(e) => {
                        println!("Error receiving message {}", e);
                        break;
                    }
                }
            }
        });
    }
}

fn clone_stream(stream: &TcpStream) -> TcpStream {
    stream.try_clone().expect("Error cloning stream")
}


#[cfg(test)]
mod client_tests {
    
}
