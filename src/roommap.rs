use std::collections::HashMap;
use std::sync::mpsc::Sender;


struct Room {
    name: String,
    members: Vec<Sender<String>>,
}

impl Room {
    pub fn new(name: String) -> Self {
        Room {
            name: name,
            members: Vec::new(),
        }
    }

    pub fn members(&self) -> &Vec<Sender<String>> {
        &self.members
    }
}

// Key: room id
// Value: Room
pub type RoomMap = HashMap<usize, Room>;