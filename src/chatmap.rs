use std::collections::HashMap;
use std::sync::mpsc::Sender;

use message::Message;

pub type ChatMap = HashMap<String, ClientInfo>;

pub struct ClientInfo {
    pub partner: Option<String>,
    pub sender_to_client: Sender<Message>
}

pub fn is_valid_username(chat_map: &ChatMap, username: String) -> bool {
	username.len() > 0  && &username[0..1] != "/" && !chat_map.contains_key(&username)
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