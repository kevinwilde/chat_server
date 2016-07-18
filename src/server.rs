use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::Sender;

use message::Message;
use roommap::{Room, RoomMap};
use usermap::UserMap;

extern crate time;

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

    pub fn welcome_user(stream: TcpStream) {
        let mut stream = stream;
        let welcome_msg = "Welcome to Smazy\nPlease enter a username:\n".to_string();
        stream.write(&welcome_msg.into_bytes()).expect("Error writing to stream");
    }

    /// Add user to server
    /// Fails if a user with the same name already exists
    /// Return boolean indicating success/failure
    pub fn add_user(&mut self, name: String) -> bool {
        if !self.users.contains_key(&name) {
            self.users.insert(name, 0);
            return true;
        }
        return false;
    }

    /// Send message to all users in the room that the sender is in
    pub fn send_message(&self, msg: Message) {
        if let Some(room) = self.rooms.get(&msg.room_id()) {
            let members = room.members();
            let output = msg.from().to_string() + ": " + msg.content() + "\n";
            for (name, sndr) in members {
                if name != msg.from() {
                    sndr.send(output.to_string()).unwrap();
                }
            }
        }
    }

    /// Add a user to a chatroom
    /// Fails if room_id is invalid
    /// Return boolean indicating success/failure
    pub fn join_room(&mut self, room_id: usize, user: String, sndr: Sender<String>) -> bool {
        let mut valid = false;
        if let Some(room) = self.rooms.get_mut(&room_id) {
            room.add_member(user.to_string(), sndr);
            valid = true;
        }
        if valid {
            let room_name  = &self.rooms.get(&room_id).unwrap().name();
            let join_msg = user.to_string() + " has joined " + room_name + ".\n";
            let msg = Message::new(time::now().asctime().to_string(),
                                   "Server".to_string(),
                                   room_id,
                                   join_msg);
            self.send_message(msg);
        }
        valid
    }

    /// Create a new chatroom
    /// Return room_id
    pub fn create_room(&mut self, room_name: String) -> usize {
        let room_id = self.rooms.len() + 1;
        self.rooms.insert(room_id, Room::new(room_name));
        room_id
    }

    /// Remove a user from a chatroom
    pub fn leave_room(&mut self, room_id: usize, user: String) {
        if let Some(room) = self.rooms.get_mut(&room_id) {
            room.remove_member(user);
        }
    }

    /// Write a list of the available chatrooms to a stream
    pub fn display_rooms(&self, stream: TcpStream) {
        let mut stream = stream;
        for (room_id, room) in &self.rooms {
            write!(stream, "{}: {}", room_id, room.name()).unwrap();
        }
    }
}