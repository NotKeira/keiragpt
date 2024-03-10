use std::io;
use std::ops::ControlFlow;

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
            println!("{}", variables);
            let response = CommandHandler::handle(command, &variables);
            if response: ControlFlow::Break(()) {
                println!("KeiraGPT: Goodbye!");
                break;
            } else if response == "I don't understand that command." {
                let response = reply(input);
            } else {
                let response = response.to_string();
            }
            println!("KeiraGPT: {}", response);
            logger::log(input, &response);
        }
    }
}

