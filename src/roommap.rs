use std::collections::HashMap;
use std::sync::mpsc::Sender;

pub struct Room {
    name: String,
    members: HashMap<String, Sender<String>>,
}

impl Room {
    pub fn new(name: String) -> Self {
        Room {
            name: name,
            members: HashMap::new(),
        }
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn members(&self) -> &HashMap<String, Sender<String>> {
        &self.members
    }

    /// Add a user to the room
    pub fn add_member(&mut self, user: String, sndr: Sender<String>) {
        self.members.insert(user, sndr);
    }

    /// Remove a user from the room
    pub fn remove_member(&mut self, user: String) {
        self.members.remove(&user);
    }

    /// Return number of members in the room
    pub fn size(&self) -> usize {
        self.members.len()
    }
}

/// Key: room id
/// Value: Room
pub type RoomMap = HashMap<usize, Room>;