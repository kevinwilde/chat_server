pub enum Command {
    Quit,
    Logoff,
    Unrecognized
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Command::Quit, &Command::Quit) => true,
            (&Command::Logoff, &Command::Logoff) => true,
            (&Command::Unrecognized, &Command::Unrecognized) => true,
            _ => false
        }
    }
}