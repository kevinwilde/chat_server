pub struct Message {
    date: String,
    from: String,
    to: String,
    content: String
}

impl Message {
    pub fn new(date: String, from: String, to: String, content: String) -> Message {
        Message {
            date: date,
            from: from,
            to: to,
            content: content
        }
    }

    pub fn get_date(&self) -> String {
        (*self.date).to_string()
    }

    pub fn get_from(&self) -> String {
        (*self.from).to_string()
    }

    pub fn get_to(&self) -> String {
        (*self.to).to_string()
    }

    pub fn get_content(&self) -> String {
        (*self.content).to_string()
    }
}
