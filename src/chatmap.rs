use std::collections::HashMap;
use std::sync::mpsc::Sender;

use message::Message;

pub type ChatMap = HashMap<String, ClientInfo>;

pub struct ClientInfo {
    pub partner: Option<String>,
    pub sender_to_client: Sender<Message>
}