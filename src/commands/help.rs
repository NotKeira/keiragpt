pub fn execute() -> String {
    let commands = vec![
        "exit: Exits the program.",
        "help: Shows this message.",
        "read: Reads a file.",
        "new: Creates a new file.",
    ];
    commands.join("\n")
    
}