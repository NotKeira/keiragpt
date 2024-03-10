pub fn reply(input: &str) -> &'static str {
    match input {
        "hello" => "Hi there!",
        "how are you" => "I'm doing okay, thank you! How about you?",
        "whats your name" => "I'm KeiraGPT!",
        "whats your favorite color" => "I don't have a favorite color, but I like the color blue!",
        "whats your favorite food" => "I don't eat, but I like the idea of pizza!",
        "whats your favorite movie" => "I don't watch movies, but I like the idea of Star Wars!",
        "whats your favorite song" => "I don't listen to music, but I like the idea of Bohemian Rhapsody!",
        "whats your favorite book" => "I don't read, but I like the idea of Harry Potter!",
        "whats your favorite animal" => "I don't have a favorite animal, but I like the idea of a cat!",
        "whats your favorite game" => "I don't play games, but I like the idea of Minecraft!",
        _ => "I'm sorry, I don't understand what you're saying!",
    }
}