use std::io::{BufRead, BufReader, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;

use chatmap::{ChatMap, ClientInfo};
use command::Command;
use message::Message;

use chatmap::{available_users, is_valid_username, quit_conversation};
use command::parse_command;

extern crate time;

pub fn create_client(stream: TcpStream, 
                     sndr_to_router: Sender<Message>, 
                     chat_map: &Arc<Mutex<ChatMap>>) {

    println!("New client");
    let mut stream = stream;
    let welcome_msg = "Welcome to Smazy\nPlease enter a username:\n".to_string();
    stream.write(&welcome_msg.into_bytes()).expect("Error writing to stream");
    stream.flush().unwrap();
    
    let mut reader = BufReader::new(stream.try_clone().expect("Error cloning stream"));
    let mut username = "".to_string();
    let (sndr_to_client, rcvr_from_router) = channel();

    loop {
        match reader.read_line(&mut username) {
            Ok(_) => {
                username = username.trim().to_string().to_lowercase();

                let mut guard = chat_map.lock().expect("Error locking chatmap");
                
                if is_valid_username(&*guard, username.to_string()) {

                    let client_info = ClientInfo{
                        partner: None, 
                        sender_to_client: sndr_to_client,
                        blocked_users: Vec::new(),
                    };

                    guard.insert(username.to_string(), client_info);
                    
                    println!("New user: {}", username);
                    break;
                }
                else {
                    let invalid_msg = "Invalid username. Please try again.\n".to_string();
                    stream.write(&invalid_msg.into_bytes()).expect("Error writing to stream");
                    username = "".to_string();
                }
            },
            Err(e) => println!("{}", e)
        }
    }

    let partner = choose_chat_partner(stream.try_clone().expect("Error cloning stream"), 
        username.to_string(), chat_map);

    chat(stream, username, partner, sndr_to_router, rcvr_from_router, chat_map);
}

fn display_available(stream: TcpStream, chat_map: &Arc<Mutex<ChatMap>>, username: String) {
    let mut stream = stream;

    let show_available_msg = "Here are the users available to chat:\n".to_string();
    stream.write(&show_available_msg.into_bytes()).expect("Error writing to stream");
    
    let avail;
    
    {
        let guard = chat_map.lock().expect("Error locking chatmap");
        avail = available_users(&*guard, username);
    }

    for user in avail {
        stream.write(&(user + "\n").to_string().into_bytes()).expect("Error writing to stream");
    }

    let select_msg = "Select who you want to chat with:\n".to_string();
    stream.write(&select_msg.into_bytes()).expect("Error writing to stream");
}

fn choose_chat_partner(stream: TcpStream, 
                       username: String, 
                       chat_map: &Arc<Mutex<ChatMap>>) -> String {
    
    // Used to notify you if someone else starts chat with you
    // To begin, you have no partner
    let no_partner = Arc::new(Mutex::new(true));

    // Allow you to start a chat
    {
        let chat_map = chat_map.clone();
        let no_partner = no_partner.clone();
        let username = username.to_string();
        let stream = stream.try_clone().expect("Error cloning stream");

        thread::spawn(move|| {

            loop {
                {
                    // Break if other thread has set no_partner to false
                    let guard = no_partner.lock().unwrap();
                    if !*guard {
                        break;
                    }
                }

                display_available(stream.try_clone().expect("Error cloning stream"), 
                    &chat_map, username.to_string());
                
                let mut reader = BufReader::new(stream.try_clone().expect("Error cloning stream"));
                let mut partner = "".to_string();
                
                match reader.read_line(&mut partner) {
                    Ok(_) => {
                        partner = partner.trim().to_string().to_lowercase();

                        if try_select_partner(&chat_map, username.to_string(), partner.to_string()) {
                            // If we successfully choose partner, 
                            //   break out of this loop and end this thread.
                            //   Other thread will find that we have partner now
                            //   and will return that partner
                            break;
                        }
                    },
                    Err(e) => println!("{}", e)
                }
            }
        });
    }

    let handle;
    
    // Check if someone else has started a chat with you
    {
        let chat_map = chat_map.clone();
        let no_partner = no_partner.clone();
        let username = username.to_string();
        let mut stream = stream;

        handle = thread::spawn(move|| {

            let mut n;

            {
                let guard = chat_map.lock().expect("Error locking chatmap");
                n = available_users(&*guard, username.to_string()).len();
            }

            loop {
                let new_n;

                {
                    let guard = chat_map.lock().expect("Error locking chatmap");
                    new_n = available_users(&*guard, username.to_string()).len();
                }

                // Re-display available users if it changes
                if n != new_n {
                    display_available(stream.try_clone().expect("Error cloning stream"), 
                        &chat_map, username.to_string());
                    n = new_n;
                }

                let guard = chat_map.lock().expect("Error locking chatmap");

                if let &Some(ref p) = &guard.get(&username).unwrap().partner {
                    println!("{} chatting with {}", username, p);
                    
                    let chatting_msg = "Now chatting with ".to_string() 
                                        + p + ". Press enter to start chatting.\n";
                    stream.write(&chatting_msg.into_bytes()).expect("Error writing to stream");
                    
                    {
                        let mut guard = no_partner.lock().unwrap();
                        *guard = false;
                    }
                    return p.to_string();
                }
            }
        });
    }

    // Return partner
    return handle.join().unwrap();
}

fn try_select_partner(chat_map: &Arc<Mutex<ChatMap>>, 
                      username: String, 
                      partner: String) -> bool {

    // Can't choose yourself as partner
    if &username[..] == &partner[..] {
        return false;
    }

    let mut guard = chat_map.lock().expect("Error locking chatmap");
    
    {
        // Not allowed to select someone you have blocked
        let my_client_info = guard.get(&username).unwrap();
        if my_client_info.blocked_users.contains(&partner) {
            return false;
        }
    }

    let success: bool;

    if let Some(clientinfo) = guard.get(&partner) {
        // If they haven't blocked you && they don't already have a partner
        if !clientinfo.blocked_users.contains(&username.to_string())
            && clientinfo.partner == None {
            success = true;
        } else {
            success = false;
        }
    } else {
        success = false
    }

    if success {
        println!("Assigning partners: {} and {}", username, partner);
        guard.get_mut(&username).unwrap().partner = Some(partner.to_string());
        guard.get_mut(&partner).unwrap().partner = Some(username.to_string());
    }
    
    success
}

fn chat(stream: TcpStream, 
        username: String, 
        partner: String, 
        sndr_to_router: Sender<Message>, 
        rcvr_from_router: Receiver<Message>,
        chat_map: &Arc<Mutex<ChatMap>>) {

    let mut partner = partner.to_string();
    
    // Send messages
    {
        let username = username.clone();
        let chat_map = chat_map.clone();
        let stream = stream.try_clone().expect("Error cloning stream");
        let reader = BufReader::new(stream.try_clone().expect("Error cloning stream"));
        
        thread::spawn(move|| {
            let mut lines = reader.lines(); 
            while let Some(Ok(line)) = lines.next() {
                println!("{}",line);

                let msg = Message::new(time::now().asctime().to_string(), 
                                       username.to_string(), 
                                       partner.to_string(), 
                                       line.to_string());
                if line.len() > 0 {
                    if &line[0..1] == "/" {
                        if let Some(p) = handle_command(stream.try_clone().expect("Error cloning stream"), 
                            &chat_map, username.to_string(), partner.to_string(), 
                            msg, &sndr_to_router) {
                            
                            partner = p;
                        }
                    } else {
                        sndr_to_router.send(msg).expect("Error sending message");
                    }
                }
            }
        });
    }

    // Receive Messages
    {
        let username = username.clone();
        thread::spawn(move|| {
            loop {
                let stream = stream.try_clone().expect("Error cloning stream");
                match rcvr_from_router.try_recv() {
                    Ok(msg) => {
                        println!("User {} received message: {}", 
                            &username, msg.content());

                        receive_message(stream, msg);
                    },

                    Err(TryRecvError::Empty) => continue,

                    Err(TryRecvError::Disconnected) => {
                        println!("User {} disconnected from router", username);
                        break;
                    }
                }
            }
        });
    }
}

fn handle_command(stream: TcpStream, 
                  chat_map: &Arc<Mutex<ChatMap>>, 
                  username: String,
                  partner: String,
                  msg: Message,
                  sndr_to_router: &Sender<Message>) -> Option<String> {

    match parse_command(msg.content().to_string()) {
                            
        Command::Quit => {
            println!("Quit command");
        
            {
                let mut guard = chat_map.lock().expect("Error locking chatmap");
                quit_conversation(&mut *guard, username.to_string());
            }

            send_quit_message(username.to_string(), partner.to_string(), &sndr_to_router);
            
            Some(choose_chat_partner(stream.try_clone().expect("Error cloning stream"), 
                username.to_string(), &chat_map))
        },

        Command::DisplayAvailable => {
            display_available(stream.try_clone().expect("Error cloning stream"), 
                &chat_map, username.to_string());
            None
        },

        Command::Block => {
            {
                let mut guard = chat_map.lock().expect("Error locking chatmap");
                quit_conversation(&mut *guard, username.to_string());
                let mut client_info = guard.get_mut(&username).unwrap();
                client_info.blocked_users.push(partner.to_string());
            }

            send_block_message(username.to_string(), partner.to_string(), &sndr_to_router);
            
            Some(choose_chat_partner(stream.try_clone().expect("Error cloning stream"), 
                username.to_string(), &chat_map))
        },

        Command::Logoff => {
            println!("Logging off {}", username);

            {
                let mut guard = chat_map.lock().expect("Error locking chatmap");
                quit_conversation(&mut *guard, username.to_string());
                guard.remove(&username).expect("Error removing from chatmap");
            }

            send_logoff_message(username.to_string(), partner.to_string(), &sndr_to_router);

            stream.shutdown(Shutdown::Both).expect("Error shutting down stream");

            None
        }

        Command::Unrecognized => {
            println!("Unrecognized command");
            None
        }
    }

}

fn send_quit_message(username: String, 
                     partner: String, 
                     sndr_to_router: &Sender<Message>) {

    let quit_msg = "I have quit out of the conversation. \
                    Type /q to quit.".to_string();
                                
    let msg = Message::new(time::now().asctime().to_string(), 
                           username, partner, quit_msg);
    
    sndr_to_router.send(msg).expect("Error sending message");
}

fn send_block_message(username: String, 
                      partner: String, 
                      sndr_to_router: &Sender<Message>) {

    let block_msg = "I have blocked you and quit out of the conversation. \
                     Type /q to quit.".to_string();
                                
    let msg = Message::new(time::now().asctime().to_string(), 
                           username, partner, block_msg);
    
    sndr_to_router.send(msg).expect("Error sending message");
}

fn send_logoff_message(username: String, 
                      partner: String, 
                      sndr_to_router: &Sender<Message>) {

    let logoff_msg = "I have logged off. \
                      Type /q to quit.".to_string();
                                
    let msg = Message::new(time::now().asctime().to_string(), 
                           username, partner, logoff_msg);
    
    sndr_to_router.send(msg).expect("Error sending message");
}

fn receive_message(stream: TcpStream, msg: Message) {
    let mut stream = stream;
    let output = msg.from().to_string() + ": " + &msg.content()[..] + "\n";
    stream.write(&output.into_bytes()).expect("Error writing to stream");
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