pub fn execute(input: &str) -> String {
    let input = input.to_lowercase();
    let mut result = String::new();
    for c in input.chars() {
        if c.is_ascii_alphanumeric() || c.is_whitespace() {
            result.push(c);
        }
    }
    println!("Stripped: {}", result);
    result
}