pub fn reply(input: String) -> &'static str {
    match input.as_str() {
        "hello" => "Hi there!",
        "how are you" => "I'm doing okay, thank you! How about you?",
        "whats your name" => "I'm KeiraGPT!",
        "whats your favourite colour" => "I don't have a favourite colour, but I like the colour blue!",
        "whats your favourite food" => "I don't eat, but I like the idea of pizza!",
        "whats your favourite movie" => "I don't watch movies, but I like the idea of Star Wars!",
        "whats your favourite song" => "I don't listen to music, but I like the idea of Bohemian Rhapsody!",
        "whats your favourite book" => "I don't read, but I like the idea of Harry Potter!",
        "whats your favourite animal" => "I don't have a favourite animal, but I like the idea of a cat!",
        "whats your favourite game" => "I don't play games, but I like the idea of Minecraft!",
        _ => "I'm sorry, I don't understand what you're saying!",
    }
}