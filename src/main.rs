use std::io;

use respond::reply;

mod respond;
mod logger;
mod strip;

fn main() {
    println!("Welcome to keira's attempt of a chatbot in rust lol");
    println!("Type 'exit' to stop.");

    loop {
        {
            println!("You: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            let input = input.trim();
            if input.to_lowercase() == "exit" {
                println!("KeiraGPT: Goodbye!");
                break;
            }

            let response = reply(&strip::execute(&input));
            println!("KeiraGPT: {}", response);

            logger::log(input, &response);
        }
    }
}

