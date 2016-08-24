#[derive(Debug)]
pub enum Command {
    Quit,
    DisplayRooms,
    Help,
    Logoff,
    Unrecognized
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Command::Quit, &Command::Quit) => true,
            (&Command::DisplayRooms, &Command::DisplayRooms) => true,
            (&Command::Help, &Command::Help) => true,
            (&Command::Logoff, &Command::Logoff) => true,
            (&Command::Unrecognized, &Command::Unrecognized) => true,
            _ => false
        }
    }
}

pub fn parse_command(cmd: String) -> Command {
    let cmd = cmd.to_lowercase();
    if cmd == "/q" || cmd == "/quit" {
        Command::Quit
    } else if cmd == "/list" {
        Command::DisplayRooms
    } else if cmd == "/help" {
        Command::Help
    } else if cmd == "/logoff" {
        Command::Logoff
    } else {
        Command::Unrecognized
    }
}

#[cfg(test)]
mod command_tests {

    use super::{Command, parse_command};

    #[test]
    fn parse_command_test_q() {
        let cmd = "/q".to_string();
        assert_eq!(Command::Quit, parse_command(cmd));
    }

    #[test]
    fn parse_command_test_quit() {
        let cmd = "/quit".to_string();
        assert_eq!(Command::Quit, parse_command(cmd));
    }

    #[test]
    fn parse_command_test_list() {
        let cmd = "/list".to_string();
        assert_eq!(Command::DisplayRooms, parse_command(cmd));
    }

    #[test]
    fn parse_command_test_help() {
        let cmd = "/help".to_string();
        assert_eq!(Command::Help, parse_command(cmd));
    }

    #[test]
    fn parse_command_test_logoff() {
        let cmd = "/logoff".to_string();
        assert_eq!(Command::Logoff, parse_command(cmd));
    }

    #[test]
    fn parse_command_test_unrecognized() {
        let cmd = "/hello".to_string();
        assert_eq!(Command::Unrecognized, parse_command(cmd));
        let cmd = "/blah".to_string();
        assert_eq!(Command::Unrecognized, parse_command(cmd));
        let cmd = "/a".to_string();
        assert_eq!(Command::Unrecognized, parse_command(cmd));
        let cmd = "/".to_string();
        assert_eq!(Command::Unrecognized, parse_command(cmd));
        let cmd = "quit".to_string();
        assert_eq!(Command::Unrecognized, parse_command(cmd));
        let cmd = "list".to_string();
        assert_eq!(Command::Unrecognized, parse_command(cmd));
        let cmd = "help".to_string();
        assert_eq!(Command::Unrecognized, parse_command(cmd));
        let cmd = "logoff".to_string();
        assert_eq!(Command::Unrecognized, parse_command(cmd));
    }
}