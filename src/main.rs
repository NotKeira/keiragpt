// disable dead code warnings
#![allow(dead_code)]
use std::io;

use command_handler::CommandHandler;

use crate::command_handler::Error;
use crate::helpers::strip;

mod command_handler;
mod commands;
mod helpers;


// random shit

/* 
            template flag module = {
            flags = ["h"],
            args = ["module", "command"],
            }
             */

fn main() {
    println!("Keewee, your personal assistant designed to help you with simple tasks you don't want to do!\nBy NqtKeira - 2024 :)");
    println!("Type 'exit' to stop.");

    loop {
        {
            println!("You: ");
            let mut input: String = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            input.trim().to_string();
            let vargs: Vec<&str> = input.split_whitespace().skip(1).collect();
            let parameters = strip::split_flags(vargs);

            let command: String = input.split_whitespace().next().unwrap().to_string();
            let result: Result<String, Error> = CommandHandler::handle(command, parameters);
            let response: String;
            match result {
                Ok(value) if &value == "break" => {
                    println!("Keewee: Goodbye!");
                    break;
                }
                Ok(value) => response = value,
                Err(_) => response = "I don't understand that command.".to_string(),
            }

            println!("Keewee: {}", response);
        }
    }
}