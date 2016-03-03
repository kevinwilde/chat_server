use std::collections::HashMap;
use std::sync::mpsc::Sender;

use message::Message;

pub type ChatMap = HashMap<String, ClientInfo>;

pub struct ClientInfo {
    pub partner: Option<String>,
    pub sender_to_client: Sender<Message>,
    pub blocked_users : Vec<String>
}

pub fn is_valid_username(chat_map: &ChatMap, username: String) -> bool {
	username.len() > 0  && &username[0..1] != "/" && !chat_map.contains_key(&username)
}

pub fn available_users(chat_map: &ChatMap, username: String) -> Vec<String> {
	let mut v = Vec::new();
    let my_client_info = chat_map.get(&username).unwrap();
    let my_blocked_users = &my_client_info.blocked_users;

	for (name, client_info) in chat_map.iter() {
        if &name[..] != &username[..] 
          && client_info.partner == None  
          && !client_info.blocked_users.contains(&username) 
          && !my_blocked_users.contains(name) {
            v.push(name.to_string());
        }
    }
    v
}

pub fn quit_conversation(chat_map: &mut ChatMap, username: String) {
    chat_map.get_mut(&username).unwrap().partner = None;
}

#[cfg(test)]
mod chatmap_tests {
	
    use std::sync::mpsc::channel;
    use super::{ChatMap, ClientInfo};
    use super::is_valid_username;

	#[test]
    fn is_valid_username_test_valid() {
    	let cm = fixture();
    	assert!(is_valid_username(&cm, "f".to_string()));    	
    	assert!(is_valid_username(&cm, "g".to_string()));    	
    	assert!(is_valid_username(&cm, "7".to_string()));
    	assert!(is_valid_username(&cm, ".".to_string()));
    	assert!(is_valid_username(&cm, "hello".to_string()));
    }

    #[test]
	fn is_valid_username_test_invalid() {
    	let cm = fixture();
    	assert!(!is_valid_username(&cm, "e".to_string()));
    	assert!(!is_valid_username(&cm, "".to_string()));
    	assert!(!is_valid_username(&cm, "/f".to_string()));    	
    	assert!(!is_valid_username(&cm, "/hello".to_string()));
    }

    use super::available_users;

    #[test]
    fn available_users_test_1() {
    	let cm = fixture();
    	let avail = available_users(&cm, "a".to_string());
    	assert!(avail.contains(&"b".to_string()));
    	assert!(avail.contains(&"c".to_string()));
    	assert!(avail.contains(&"d".to_string()));
    	assert!(avail.contains(&"e".to_string()));
    }

    #[test]
    fn available_users_test_2() {
    	let mut cm = fixture();

    	// Set c and d as partners
    	cm.get_mut(&"c".to_string()).unwrap().partner = Some("d".to_string());
    	cm.get_mut(&"d".to_string()).unwrap().partner = Some("c".to_string());

    	let avail = available_users(&cm, "a".to_string());
    	assert!(avail.contains(&"b".to_string()));
    	assert!(!avail.contains(&"c".to_string()));
    	assert!(!avail.contains(&"d".to_string()));
    	assert!(avail.contains(&"e".to_string()));
    }

    // use super::end_conversation;

    // #[test]
    // fn end_conversation_test_1() {
    //     let mut cm = fixture();
        
    //     // Set a and b as partners
    //     cm.get_mut(&"a".to_string()).unwrap().partner = Some("b".to_string());
    //     cm.get_mut(&"b".to_string()).unwrap().partner = Some("a".to_string());

    //     end_conversation(&mut cm, "a".to_string(), "b".to_string());
        
    //     assert_eq!(None, cm.get(&"a".to_string()).unwrap().partner);            
    //     assert_eq!(None, cm.get(&"b".to_string()).unwrap().partner);
    // }

    // #[test]
    // fn end_conversation_test_2() {
    //     let mut cm = fixture();

    //     // Set a and b as partners
    //     cm.get_mut(&"a".to_string()).unwrap().partner = Some("b".to_string());
    //     cm.get_mut(&"b".to_string()).unwrap().partner = Some("a".to_string());
        
    //     // Set c and d as partners
    //     cm.get_mut(&"c".to_string()).unwrap().partner = Some("d".to_string());
    //     cm.get_mut(&"d".to_string()).unwrap().partner = Some("c".to_string());
        
    //     end_conversation(&mut cm, "a".to_string(), "b".to_string());
        
    //     assert_eq!(None, cm.get(&"a".to_string()).unwrap().partner);
    //     assert_eq!(None, cm.get(&"b".to_string()).unwrap().partner);
    //     assert_eq!(Some("d".to_string()), cm.get(&"c".to_string()).unwrap().partner);
    //     assert_eq!(Some("c".to_string()), cm.get(&"d".to_string()).unwrap().partner);
    // }

    // #[test]
    // fn end_conversation_test_3() {
    //     let mut cm = fixture();
        
    //     // Set a and b as partners
    //     cm.get_mut(&"a".to_string()).unwrap().partner = Some("b".to_string());
    //     cm.get_mut(&"b".to_string()).unwrap().partner = Some("a".to_string());

    //     end_conversation(&mut cm, "a".to_string(), "b".to_string());
        
    //     // Set a and c as partners
    //     cm.get_mut(&"a".to_string()).unwrap().partner = Some("c".to_string());
    //     cm.get_mut(&"c".to_string()).unwrap().partner = Some("a".to_string());        

    //     end_conversation(&mut cm, "b".to_string(), "a".to_string());

    //     assert_eq!(Some("c".to_string()), cm.get(&"a".to_string()).unwrap().partner);
    //     assert_eq!(None, cm.get(&"b".to_string()).unwrap().partner);
    //     assert_eq!(Some("a".to_string()), cm.get(&"c".to_string()).unwrap().partner);
    // }


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