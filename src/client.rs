use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;

use chatmap::*;
use command::{Command, parse_command};
use message::Message;

extern crate time;

pub fn create_client(stream: TcpStream, 
                     sender_to_router: Sender<Message>, 
                     chat_map: &Arc<Mutex<ChatMap>>) {

    println!("New client");
    let mut stream = stream;
    
    stream.write(&"Welcome to Smazy\n".to_string().into_bytes()).unwrap();
    stream.write(&"Please enter a username:\n".to_string().into_bytes()).unwrap();
    stream.flush().unwrap();
    
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut username = "".to_string();
    let (sender_to_client, receiver_from_router) = channel();

    loop {
        match reader.read_line(&mut username) {
            Ok(_) => {
                username = username.trim().to_string();
                
                {
                    let mut guard = chat_map.lock().unwrap();
                    if is_valid_username(&*guard, username.to_string()) {

                        let client_info = ClientInfo{
                            partner: None, 
                            sender_to_client: sender_to_client
                        };
                        guard.insert(username.to_string(), client_info);
                        
                        println!("New user: {}", username.to_string());
                        break;
                    }
                    else {
                        stream.write(&"Invalid username. Please try again.\n".to_string().into_bytes()).unwrap();
                        username = "".to_string();
                    }
                }
            },
            Err(e) => println!("{}", e)
        }
    }

    let partner = choose_chat_partner(stream.try_clone().unwrap(), 
        username.to_string(), chat_map);

    chat(stream, username, partner, sender_to_router, receiver_from_router, chat_map);
}

fn display_available(stream: TcpStream, chat_map: &Arc<Mutex<ChatMap>>, username: String) {
    let mut stream = stream;

    let show_available_msg = "Here are the users available to chat:\n".to_string();
    stream.write(&show_available_msg.into_bytes()).unwrap();
    
    {
        let guard = chat_map.lock().unwrap();
        for (name, client_info) in guard.iter() {
            if &name[..] != &username[..] && client_info.partner == None {
                stream.write(&name.to_string().into_bytes()).unwrap();
                stream.write(&"\n".to_string().into_bytes()).unwrap();
            }
        }
    }
}

fn choose_chat_partner(stream: TcpStream, 
                       username: String, 
                       chat_map: &Arc<Mutex<ChatMap>>) -> String {
    
    display_available(stream.try_clone().unwrap(), &chat_map, username.to_string());
    
    let mut stream = stream;
    
    let select_msg = "Select who you want to chat with:\n".to_string();
    stream.write(&select_msg.into_bytes()).unwrap();
    
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

fn try_select_partner(chat_map: &Arc<Mutex<ChatMap>>, 
                      username: String, 
                      partner: String) -> bool {

    let success: bool;
    {
        let mut guard = chat_map.lock().unwrap();
        
        match guard.get_mut(&partner) {
            Some(clientinfo) => {
                if &partner[..] != &username[..]
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

fn chat(stream: TcpStream, 
        username: String, 
        partner: String, 
        sender_to_router: Sender<Message>, 
        receiver_from_router: Receiver<Message>,
        chat_map: &Arc<Mutex<ChatMap>>) {

    let mut partner = partner.to_string();
    
    // Send messages
    {
        let username = username.clone();
        let chat_map = chat_map.clone();
        let stream = stream.try_clone().unwrap();
        let reader = BufReader::new(stream.try_clone().unwrap());
        thread::spawn(move|| {
            let mut lines = reader.lines(); 
            while let Some(Ok(line)) = lines.next() {
                println!("{}",line);

                let msg = Message::new(time::now().asctime().to_string(), 
                                       username.to_string(), 
                                       partner.to_string(), 
                                       line.to_string());

                if &line[0..1] == "/" {
                    match parse_command(msg.content().to_string()) {
                        Command::Quit => {
                            println!("Quit command");
                            quit_conversation(&chat_map, username.to_string(), partner.to_string());
                            partner = choose_chat_partner(stream.try_clone().unwrap(), username.to_string(), &chat_map);
                        },
                        Command::DisplayAvailable => {
                            display_available(stream.try_clone().unwrap(), &chat_map, username.to_string());
                        },
                        Command::Logoff => println!("Logoff command"),
                        Command::Unrecognized => println!("Unrecognized command")
                    }
                } else {
                    sender_to_router.send(msg).unwrap();
                }
            }
        });
    }

    // Receive Messages
    {
        let username = username.clone();
        thread::spawn(move|| {
            loop {
                let stream = stream.try_clone().unwrap();
                match receiver_from_router.try_recv() {
                    Ok(msg) => {
                        println!("User {} received message: {}", 
                            &username, msg.content());

                        receive_message(stream, msg);
                    },

                    Err(TryRecvError::Empty) => continue,

                    Err(TryRecvError::Disconnected) => 
                        panic!("User {} disconnected from router", &username)
                }
            }
        });
    }
}

fn receive_message(stream: TcpStream, msg: Message) {
    let mut stream = stream;
    let output = msg.from().to_string() + ": " + &msg.content()[..] + "\n";
    stream.write(&output.into_bytes()).unwrap();
}

fn quit_conversation(chat_map: &Arc<Mutex<ChatMap>>, username: String, partner: String) {
    let mut guard = chat_map.lock().unwrap();
    guard.get_mut(&username).unwrap().partner = None;
    guard.get_mut(&partner).unwrap().partner = None;
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
    fn try_select_partner_test_succes_3() {
        let cm = Arc::new(Mutex::new(fixture()));

        // Success: a and b become partners
        assert!(try_select_partner(&cm, "a".to_string(), "b".to_string()));

        // Success: b can choose a when they are already partners
        //   (This allows someone to confirm a chat request)
        assert!(try_select_partner(&cm, "b".to_string(), "a".to_string()));
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

        // Fail: Can't choose self as partner
        assert!(!try_select_partner(&cm, "c".to_string(), "c".to_string()));
    }

    #[test]
    fn try_select_partner_test_fail_3() {
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

    use super::quit_conversation;

    #[test]
    fn quit_conversation_test_1() {
        let cm = Arc::new(Mutex::new(fixture()));
        if try_select_partner(&cm, "a".to_string(), "b".to_string()) {
            quit_conversation(&cm, "a".to_string(), "b".to_string());
            let guard = cm.lock().unwrap();
            assert_eq!(None, guard.get(&"a".to_string()).unwrap().partner);            
            assert_eq!(None, guard.get(&"b".to_string()).unwrap().partner);
        } else {
            assert!(false); // Don't pass if try_select_partner fails
        }
    }

    #[test]
    fn quit_conversation_test_2() {
        let cm = Arc::new(Mutex::new(fixture()));
        let ab = try_select_partner(&cm, "a".to_string(), "b".to_string());
        let cd = try_select_partner(&cm, "c".to_string(), "d".to_string());
        if ab && cd {
            quit_conversation(&cm, "a".to_string(), "b".to_string());
            let guard = cm.lock().unwrap();
            assert_eq!(None, guard.get(&"a".to_string()).unwrap().partner);
            assert_eq!(None, guard.get(&"b".to_string()).unwrap().partner);
            assert_eq!(Some("d".to_string()), guard.get(&"c".to_string()).unwrap().partner);
            assert_eq!(Some("c".to_string()), guard.get(&"d".to_string()).unwrap().partner);
        } else {
            assert!(false); // Don't pass if try_select_partner fails
        }
    }

    fn fixture() -> ChatMap {
        let mut cm = ChatMap::new();
        let (sender_to_a, _) = channel();
        cm.insert("a".to_string(), ClientInfo{
            partner: None,
            sender_to_client: sender_to_a
        });
        let (sender_to_b, _) = channel();
        cm.insert("b".to_string(), ClientInfo{
            partner: None,
            sender_to_client: sender_to_b
        });
        let (sender_to_c, _) = channel();
        cm.insert("c".to_string(), ClientInfo{
            partner: None,
            sender_to_client: sender_to_c
        });
        let (sender_to_d, _) = channel();
        cm.insert("d".to_string(), ClientInfo{
            partner: None,
            sender_to_client: sender_to_d
        });
        let (sender_to_e, _) = channel();
        cm.insert("e".to_string(), ClientInfo{
            partner: None,
            sender_to_client: sender_to_e
        });
        cm
    }
}