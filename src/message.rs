use command::Command;

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

    pub fn date(&self) -> &String {
        &self.date
    }

    pub fn from(&self) -> &String {
        &self.from
    }

    pub fn to(&self) -> &String {
        &self.to
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn command(&self) -> Command {
        if self.content() == "/q" {
            Command::Quit
        } else if self.content() == "/logoff" {
            Command::Logoff
        } else {
            Command::Unrecognized
        }

    }
}
