use std::io::{BufRead, BufReader, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

// use chatmap::{ChatMap, ClientInfo};
// use command::Command;
use message::Message;
use server::Server;

// use chatmap::{available_users, is_valid_username, quit_conversation};
// use command::parse_command;

extern crate time;

pub fn create_client(stream: TcpStream, server: &Arc<Mutex<Server>>) {
    println!("New client");    
    welcome_user(clone_stream(&stream));
    let username = choose_username(clone_stream(&stream), server);   
    display_instructions(clone_stream(&stream));
    chat(stream, server, username);
}

fn welcome_user(stream: TcpStream) {
    let mut stream = stream;
    let welcome_msg = "Welcome to Smazy\nPlease enter a username:\n".to_string();
    stream.write(&welcome_msg.into_bytes()).expect("Error writing to stream");
    // stream.flush().unwrap();
}

fn display_instructions(stream: TcpStream) {
    let mut stream = stream;
    let instructions = "Instructions:\nThis will be filled in later\n".to_string();
    stream.write(&instructions.into_bytes()).expect("Error writing to stream");
    // stream.flush().unwrap();
}

fn choose_username(stream: TcpStream, server: &Arc<Mutex<Server>>) -> String {
    let mut reader = BufReader::new(clone_stream(&stream));
    let mut username = "".to_string();

    loop {
        match reader.read_line(&mut username) {
            Ok(_) => {
                username = username.trim().to_string().to_lowercase();

                if server.lock().unwrap().add_user(username.to_string()) {
                    break
                }
                else {
                    let mut stream = clone_stream(&stream);
                    let invalid_msg = "Invalid username. Please try again.\n".to_string();
                    stream.write(&invalid_msg.into_bytes()).expect("Error writing to stream");
                    username = "".to_string();
                }
            },
            Err(e) => println!("{}", e)
        }
    }
    username.to_string()
}

fn choose_chatroom(stream: TcpStream, server: &Arc<Mutex<Server>>, sndr: Sender<String>) -> usize {
    server.lock().unwrap().display_rooms(clone_stream(&stream));
    unimplemented!();
}

fn chat(stream: TcpStream, server: &Arc<Mutex<Server>>, username: String) {
    let (sndr, rcvr) = channel();

    let room_num = choose_chatroom(clone_stream(&stream), server, sndr);
    
    // Send messages
    {
        let username = username.to_string();
        let reader = BufReader::new(clone_stream(&stream));
        let server = server.clone();

        thread::spawn(move|| {
            let mut lines = reader.lines(); 
            while let Some(Ok(line)) = lines.next() {
                if line.len() > 0 {
                    let msg = Message::new(time::now().asctime().to_string(), 
                                           username.to_string(), 
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
            loop {
                match rcvr.recv() {
                    Ok(msg) => {
                        receive_message(clone_stream(&stream), msg);
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


fn receive_message(stream: TcpStream, msg: String) {
    let mut stream = stream;
    stream.write(&msg.into_bytes()).expect("Error writing to stream");
}

fn clone_stream(stream: &TcpStream) -> TcpStream {
    stream.try_clone().expect("Error cloning stream")
}


#[cfg(test)]
mod client_tests {

    use std::sync::{Arc, Mutex};
    use std::sync::mpsc::channel;
    use chatmap::{ChatMap, ClientInfo};
    
    use super::try_select_partner;

    #[test]
    fn try_select_partner_test_success_1() {
        let cm = Arc::new(Mutex::new(fixture()));

        // Success: a and b become partners
        assert!(try_select_partner(&cm, "a".to_string(), "b".to_string()));
    }

    #[test]
    fn try_select_partner_test_success_2() {
        let cm = Arc::new(Mutex::new(fixture()));

        // Success: a and b become partners
        assert!(try_select_partner(&cm, "a".to_string(), "b".to_string()));

        // Success: c and d become partners
        assert!(try_select_partner(&cm, "c".to_string(), "d".to_string()));
    }

    #[test]
    fn try_select_partner_test_fail_1() {
        let cm = Arc::new(Mutex::new(fixture()));

        // Success: a and b become partners
        assert!(try_select_partner(&cm, "a".to_string(), "b".to_string()));

        // Fail: b already has a partner
        assert!(!try_select_partner(&cm, "c".to_string(), "b".to_string()));
    }

    #[test]
    fn try_select_partner_test_fail_2() {
        let cm = Arc::new(Mutex::new(fixture()));

        // Success: a and b become partners
        assert!(try_select_partner(&cm, "a".to_string(), "b".to_string()));

        // Fail: a already has partner
        assert!(!try_select_partner(&cm, "b".to_string(), "a".to_string()));
    }

    #[test]
    fn try_select_partner_test_fail_3() {
        let cm = Arc::new(Mutex::new(fixture()));

        // Success: a and b become partners
        assert!(try_select_partner(&cm, "a".to_string(), "b".to_string()));

        // Fail: Can't choose self as partner
        assert!(!try_select_partner(&cm, "c".to_string(), "c".to_string()));
    }

    #[test]
    fn try_select_partner_test_fail_4() {
        let cm = Arc::new(Mutex::new(fixture()));

        // Fail: f not in chatmap
        assert!(!try_select_partner(&cm, "a".to_string(), "f".to_string()));
    }

    #[test]
    fn try_select_partner_test_fail_5() {
        let cm = Arc::new(Mutex::new(fixture()));

        // Success: a and b become partners
        assert!(try_select_partner(&cm, "a".to_string(), "b".to_string()));

        // Success: c and d become partners
        assert!(try_select_partner(&cm, "c".to_string(), "d".to_string()));

        // Fail: c already has a partner
        assert!(!try_select_partner(&cm, "e".to_string(), "c".to_string()));

        // Fail: d already has a partner
        assert!(!try_select_partner(&cm, "e".to_string(), "d".to_string()));
    }

    fn fixture() -> ChatMap {
        let mut cm = ChatMap::new();
        let (sender_to_a, _) = channel();
        cm.insert("a".to_string(), ClientInfo{
            partner: None,
            sender_to_client: sender_to_a,
            blocked_users: Vec::new()
        });
        let (sender_to_b, _) = channel();
        cm.insert("b".to_string(), ClientInfo{
            partner: None,
            sender_to_client: sender_to_b,
            blocked_users: Vec::new()
        });
        let (sender_to_c, _) = channel();
        cm.insert("c".to_string(), ClientInfo{
            partner: None,
            sender_to_client: sender_to_c,
            blocked_users: Vec::new()
        });
        let (sender_to_d, _) = channel();
        cm.insert("d".to_string(), ClientInfo{
            partner: None,
            sender_to_client: sender_to_d,
            blocked_users: Vec::new()
        });
        let (sender_to_e, _) = channel();
        cm.insert("e".to_string(), ClientInfo{
            partner: None,
            sender_to_client: sender_to_e,
            blocked_users: Vec::new()
        });
        cm
    }
}
