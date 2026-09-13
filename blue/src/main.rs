use std::io::{self, Write};

fn main() {
    loop {
        print!("> ");

        // Make sure the prompt appears immediately
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        // println!("You typed: {}", input);

        if input == "exit" {
            break;
        }
    }
}
