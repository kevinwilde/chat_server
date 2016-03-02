pub enum Command {
    Quit,
    DisplayAvailable,
    Logoff,
    Unrecognized
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Command::Quit, &Command::Quit) => true,
            (&Command::DisplayAvailable, &Command::DisplayAvailable) => true,
            (&Command::Logoff, &Command::Logoff) => true,
            (&Command::Unrecognized, &Command::Unrecognized) => true,
            _ => false
        }
    }
}

pub fn parse_command(cmd: String) -> Command {
    if cmd == "/q" {
        Command::Quit
    } else if cmd == "/list" {
        Command::DisplayAvailable
    } else if cmd == "/logoff" {
        Command::Logoff
    } else {
        Command::Unrecognized
    }
}