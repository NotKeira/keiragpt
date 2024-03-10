use std::io;

use command_handler::CommandHandler;
use respond::reply;

mod respond;
mod logger;
mod strip;
mod command_handler;

mod commands;

fn main() {
    println!("KeiraGPT, your personal assistant designed to help you with simple tasks you don't want to do!\nBy NqtKeira - 2024 :)");
    println!("Type 'exit' to stop.");

    loop {
        {
            println!("You: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            let input = input.trim();
            let command = input.split_whitespace().next().unwrap();
            let variables = input.split_whitespace().skip(1).collect::<Vec<&str>>().join(" ");
            let response = CommandHandler::handle(command, &variables);
            let responding_text;
            if response=="break" {
                println!("KeiraGPT: Goodbye!");
                break;
            } else if response == "I don't understand that command." {
                responding_text = reply(strip::execute(&input)).parse().unwrap();
            } else {
                responding_text = response.to_string();
            }
            println!("KeiraGPT: {}", responding_text);
            logger::log(input, &responding_text);
        }
    }
}

