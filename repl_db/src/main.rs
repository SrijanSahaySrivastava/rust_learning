use std::io::Write;

fn main() {
    println!("Welcome to the REPL Database!");
    println!("Please input your command.");
    let mut command = String::new();
    loop {
        command.clear();
        print!("DB> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        match command.trim() {
            "help" => {
                println!("Commands:");
                println!("  help: Display this help message");
                println!("  exit: Exit the REPL");
            }
            "exit" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Unrecognized command: {}", command.trim());
            }
            
        }
    }
}
