#[derive(Debug)]
pub enum Command {
    Quit,
    DisplayAvailable,
    Block,
    Logoff,
    Unrecognized
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Command::Quit, &Command::Quit) => true,
            (&Command::DisplayAvailable, &Command::DisplayAvailable) => true,
            (&Command::Block, &Command::Block) => true,
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
        Command::DisplayAvailable
    } else if cmd == "/block" {
        Command::Block
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
        assert_eq!(Command::DisplayAvailable, parse_command(cmd));
    }

    #[test]
    fn parse_command_test_block() {
        let cmd = "/block".to_string();
        assert_eq!(Command::Block, parse_command(cmd));
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
        let cmd = "block".to_string();
        assert_eq!(Command::Unrecognized, parse_command(cmd));
        let cmd = "logoff".to_string();
        assert_eq!(Command::Unrecognized, parse_command(cmd));
    }
}