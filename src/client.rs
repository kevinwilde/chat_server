use std::io::{BufRead, BufReader, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use command::Command;
use message::Message;
use server::Server;

use command::parse_command;

pub fn run(stream: TcpStream, server: &Arc<Mutex<Server>>) {
    println!("New client");    
    Server::welcome_user(clone_stream(&stream));
    let mut client = Client::new(clone_stream(&stream));
    client.choose_username(server);
    Server::display_instructions(clone_stream(&stream));
    client.chat(server);
}

struct Client {
    username: String,
    stream: TcpStream,
}

impl Client {
    fn new(stream: TcpStream) -> Self {
        Client {
            username: "".to_string(),
            stream: stream,
        }
    }

    fn clone(&self) -> Self {
        Client {
            username: self.username.to_string(),
            stream: clone_stream(&self.stream),
        }
    }

    fn choose_username(&mut self, server: &Arc<Mutex<Server>>) {
        let mut reader = BufReader::new(clone_stream(&self.stream));
        let mut username = "".to_string();

        while self.username.len() == 0 {
            match reader.read_line(&mut username) {
                Ok(_) => {
                    username = username.trim().to_string().to_lowercase();

                    if server.lock().unwrap().add_user(username.to_string()) {
                        self.username = username.to_string();
                    }
                    else {
                        let mut stream = clone_stream(&self.stream);
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

    fn choose_chatroom(&self, server: &Arc<Mutex<Server>>, sndr: Sender<String>) -> usize {
        let mut stream = clone_stream(&self.stream);
        let msg = "Available rooms:\n".to_string();
        stream.write(&msg.into_bytes()).expect("Error writing to stream");

        server.lock().unwrap().display_rooms(clone_stream(&stream));

        let msg = "\nEnter the room number of the room you wish to join, \
                    or type \"new\".\n".to_string();
        stream.write(&msg.into_bytes()).expect("Error writing to stream");

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
                            server.join_room(room_num, self.username.to_string(),
                                             sndr.clone());
                            return room_num;
                        }
                    }
                    else if let Ok(room_num) = choice.parse() {
                        if server.lock().unwrap()
                                .join_room(room_num, self.username.to_string(),
                                           sndr.clone()) {
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

    fn chat(self, server: &Arc<Mutex<Server>>) {
        let (sndr, rcvr) = channel();
        let stream = clone_stream(&self.stream);
        thread::spawn(move|| {
            receive_messages(stream, rcvr);
        });
        &self.send_messages(server, sndr);
    }

    fn send_messages(&self, server: &Arc<Mutex<Server>>, sndr: Sender<String>) {
        loop {
            let _self = self.clone();
            let room_num = self.choose_chatroom(server, sndr.clone());
            let username = self.username.to_string();
            let reader = BufReader::new(clone_stream(&self.stream));
            let server = server.clone();
            let handle = thread::spawn(move|| {
                let mut lines = reader.lines(); 
                while let Some(Ok(line)) = lines.next() {
                    if line.len() > 0 {
                        if &line[0..1] == "/" {
                            _self.handle_command(room_num, line, &server);
                        }
                        else {
                            let msg = Message::new(username.to_string(),
                                                   room_num,
                                                   line.to_string());
                            server.lock().unwrap().send_message(msg);
                        }
                    }
                }
            });
            let _ = handle.join();
        }
    }

    fn handle_command(&self, room_id: usize, command: String, server: &Arc<Mutex<Server>>) {
        match parse_command(command) {
            Command::Quit => {
                let mut server = server.lock().unwrap();
                server.leave_room(room_id, self.username.to_string());
            },
            Command::Logoff => {
                let _ = self.stream.shutdown(Shutdown::Both);
            },
            _ => unimplemented!(),
        }
    }
}

fn receive_messages(stream: TcpStream, rcvr: Receiver<String>) {
    let mut stream = stream;
    loop {
        match rcvr.recv() {
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
}

fn clone_stream(stream: &TcpStream) -> TcpStream {
    stream.try_clone().expect("Error cloning stream")
}


#[cfg(test)]
mod client_tests {
    
}
