pub struct Message {
    from: String,
    room_id: usize,
    content: String,
}

impl Message {
    pub fn new(from: String, room_id: usize, content: String) -> Self {
        Message {
            from: from,
            room_id: room_id,
            content: content,
        }
    }

    pub fn from(&self) -> &String {
        &self.from
    }

    pub fn room_id(&self) -> usize {
        self.room_id
    }

    pub fn content(&self) -> &String {
        &self.content
    }
}
