use crate::Algo;

#[derive(PartialEq, Debug)]
pub enum Command {
   Select(String),
   Encrypt(Option<(i8, String)>),
   Decrypt(Option<(i8, String)>),
   Key(i8),
   Message(String),
   Status,
   Exit,
   Help,
   Empty,
}

impl Command {
    fn describe(&self) -> String {
        match &self {
            Command::Select(_) => format!("Select an algo [{}]", Algo::list_algo()),
            Command::Encrypt(_) => "Encrypt according to the given algorithm.\noptional: [key] [message]".to_string(),
            Command::Decrypt(_) => "Decrypt according to the given algorithm.\noptional: [key] [message]".to_string(),
            Command::Key(_) => "[number] set the key".to_string(),
            Command::Message(_) => "[string] set the message".to_string(),
            Command::Status => "Get the status of the current algorithm".to_string(),
            Command::Exit => "Quit the application".to_string(),
            Command::Help => "Display the help message".to_string(),
            Command::Empty => "".to_string()
        }
    }
}

impl Iterator for Command {
    type Item = Command;
    fn next(&mut self) -> Option<Command> {
        match self {
            Command::Empty => Some(Command::Select("".to_string())),
            Command::Select(_) => Some(Command::Encrypt(None)),
            Command::Encrypt(_) => Some(Command::Decrypt(None)),
            Command::Decrypt(_) => Some(Command::Key(0_i8)),
            Command::Key(_) => Some(Command::Message("".to_string())),
            Command::Message(_) => Some(Command::Status),
            Command::Status => Some(Command::Exit),
            Command::Exit => Some(Command::Help),
            Command::Help => None
        }
    }
}
