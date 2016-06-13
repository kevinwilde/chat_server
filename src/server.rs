use std::io::Write;
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
        for (_, sndr) in members {
            sndr.send(output.to_string()).unwrap();
        }
    }

    pub fn join_room(&mut self, room_id: usize, user: String, sndr: Sender<String>) {
        self.rooms.get_mut(&room_id).unwrap().add_member(user, sndr);
    }

    pub fn create_room(&self) {
        unimplemented!();
    }

    pub fn leave_room(&mut self, room_id: usize, user: String) {
        self.rooms.get_mut(&room_id).unwrap().remove_member(user);
    }

    pub fn display_rooms(&self, stream: TcpStream) {
        let mut stream = stream;
        for (room_id, room) in &self.rooms {
            write!(stream, "{}: {}", room_id, room.name()).unwrap();
        }
    }
}