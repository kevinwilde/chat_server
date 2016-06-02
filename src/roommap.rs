use std::collections::HashMap;
use std::sync::mpsc::Sender;


struct Room {
    name: String,
    members: Vec<Sender>,
}

// Key: room id
// Value: Room
pub type RoomMap = HashMap<usize, Room>;