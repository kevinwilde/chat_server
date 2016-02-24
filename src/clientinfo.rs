use std::sync::mpsc::Sender;

use message::Message;

pub struct ClientInfo {
    pub partner: Option<String>,
    pub sender_to_client: Sender<Message>
}