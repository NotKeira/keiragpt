use std::{fs, str};

pub fn execute(location: &str) -> String {
    let file = location;
    println!("Reading file: {}", file);
    fs::read_to_string(file)
        .expect("Something went wrong reading the file")
        .to_string().to_string().to_string().to_string().to_string().to_string().to_string()
}