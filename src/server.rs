use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender};

use message::Message;
use roommap::RoomMap;
use usermap::UserMap;

pub struct Server {
    users: UserMap,
    rooms: RoomMap,
}

impl Server {
    pub fn new() -> Self {
        Server {
            users: UserMap::new(),
            rooms: RoomMap::new(),
        }
    }

    pub fn add_user(&self, user: String) -> bool {
        unimplemented!();
    }

    pub fn send_message(&self, msg: Message) {
        unimplemented!();
    }

    pub fn join_room(&self, user: String) {
        unimplemented!();
    }

    pub fn leave_room(&self, user: String) {
        unimplemented!();
    }

    pub fn display_rooms(&self, stream: TcpStream) {
        unimplemented!();
    }
}