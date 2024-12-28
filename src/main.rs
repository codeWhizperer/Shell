#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        // Uncomment this block to pass the first stage
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let command = input.trim();

        // REPL: READ-EVAL-PRINT LOOP

        // To properly implement REPL: I have to implement a loop.
        match command {
            _ => println!("{}: command not found", command),
        }
    }
}
