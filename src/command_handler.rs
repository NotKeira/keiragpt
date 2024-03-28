use crate::commands::{exit, help, calculate};
use crate::helpers::strip::Parameters;
pub struct CommandHandler;

pub enum Error {
    EmptyCommand,
}

impl CommandHandler {
    pub fn handle(command: String, _variables: Parameters) -> Result<String, Error> {
        if command.is_empty() {
            return Err(Error::EmptyCommand);
        }
        
        match command.as_str() {
            "exit" => Ok(exit::execute()),
            "calc" => Ok(calculate::execute(_variables)),
            "help" => Ok(help::execute()),
            _ => Err(Error::EmptyCommand),
        }
    }
}
