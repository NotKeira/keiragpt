
use crate::commands::{exit, help, read};

pub struct CommandHandler;

impl CommandHandler {
    pub fn handle(command: &str, variables: &str) -> String {
        if variables.is_empty() && command =="read" {
            panic!("No variables provided.")
        }
        match command {
            "exit" => exit::execute(),
            "help" => help::execute(),
            "read" => read::execute(&variables),
            _ => "I don't understand that command.".to_string(),
        }
    }
}
