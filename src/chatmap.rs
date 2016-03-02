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