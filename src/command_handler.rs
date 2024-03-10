use std::any::Any;
use crate::commands::{create, exit, help, read};

pub struct CommandHandler;

impl CommandHandler {
    pub fn handle(command: &str, variables: &str) -> String {
        match command {
            "exit" => exit::execute().to_string(),
            "help" => help::execute(),
            "read" => read::execute(variables.file_name: &str),
            // "new" => create::execute(),
            _ => "I don't understand that command.".to_string(),
        }
    }
}
