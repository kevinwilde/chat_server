pub struct Message {
    date: String,
    from: String,
    room_id: usize,
    content: String,
}

impl Message {
    pub fn new(date: String, from: String, room_id: usize, content: String) -> Self {
        Message {
            date: date,
            from: from,
            room_id: room_id,
            content: content,
        }
    }

    // pub fn date(&self) -> &String {
    //     &self.date
    // }

    pub fn from(&self) -> &String {
        &self.from
    }

    // pub fn to(&self) -> usize {
    //     &self.to
    // }

    pub fn content(&self) -> &String {
        &self.content
    }
}
