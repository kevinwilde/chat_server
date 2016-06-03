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

    pub fn add_user(&mut self, user: String) -> bool {
        if !self.users.contains_key(&user) {
            self.users.insert(user, 0);
            return true;
        }
        false
    }

    pub fn send_message(&self, msg: Message) {
        let members = self.rooms.get(&msg.room_id()).unwrap().members();
        let output = msg.from().to_string() + ": " + msg.content();
        for sndr in members {
            sndr.send(output.to_string()).unwrap();
        }
    }

    pub fn join_room(&self, user: String) {
        unimplemented!();
    }

    pub fn create_room(&self) {
        unimplemented!();
    }

    pub fn leave_room(&self, user: String) {
        unimplemented!();
    }

    pub fn display_rooms(&self, stream: TcpStream) {
        unimplemented!();
    }
}