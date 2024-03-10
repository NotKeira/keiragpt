pub  fn reply(input: &str) -> &'static str {
    match input {
        "hello" => "Hi there!",
        "how are you?" => "I'm doing okay, thank you! How about you?",
        "what's your name?" => "I'm KeiraGPT!",
        "what's your favorite color?" => "I don't have a favorite color, but I like the color blue!",
        "what's your favorite food?" => "I don't eat, but I like the idea of pizza!",
        "what's your favorite movie?" => "I don't watch movies, but I like the idea of Star Wars!",
        "what's your favorite song?" => "I don't listen to music, but I like the idea of Bohemian Rhapsody!",
        "what's your favorite book?" => "I don't read, but I like the idea of Harry Potter!",
        "what's your favorite animal?" => "I don't have a favorite animal, but I like the idea of a cat!",
        "what's your favorite game?" => "I don't play games, but I like the idea of Minecraft!",
        _ => "I'm sorry, I don't understand what you're saying!",
    }
}