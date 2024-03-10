use std::fs;

pub fn execute(location: &str) -> String {
    let file = "./storage/{}".to_owned() +location;
    fs::read_to_string(file)
        .expect("Something went wrong reading the file")
        .to_string()
}