use std::fs::OpenOptions;
use std::io::Write;

pub fn log(input: &str, response: &str) {
    let mut file = OpenOptions::new().create(true).append(true).open("./storage/chat.log").expect("Failed to open chat.log");

    writeln!(file, "You: {}", input).expect("Failed to write to chat.log");
    writeln!(file, "KeiraGPT: {}\n", response).expect("Failed to write to chat.log");
}