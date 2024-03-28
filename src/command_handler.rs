use crate::commands::{exit, help};
use crate::helpers::strip::Parameters;
pub struct CommandHandler;

pub enum Error {
    EmptyVariables,
    EmptyCommand,
}

impl CommandHandler {
    pub fn handle(command: String, variables: Parameters) -> Result<String, Error> {
        if command.is_empty() {
            return Err(Error::EmptyCommand);
        }
        
        match command.as_str() {
            "exit" => Ok(exit::execute()),
            "help" => Ok(help::execute()),
            _ => Err(Error::EmptyCommand),
        }
    }
}
