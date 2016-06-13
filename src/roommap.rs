use std::collections::HashMap;
use std::sync::mpsc::Sender;

struct Room {
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

    pub fn add_member(&mut self, user: String, sndr: Sender<String>) {
        self.members.insert(user, sndr);
    }

    pub fn remove_member(&mut self, user: String) {
        self.members.remove(&user);
    }
}

// Key: room id
// Value: Room
pub type RoomMap = HashMap<usize, Room>;