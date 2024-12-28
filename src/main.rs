#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env::args, os::unix::process, process::ExitCode};

fn main() {
    loop {
        // Uncomment this block to pass the first stage
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        // let command = input.trim();
        let trimmed = input.trim();

        // REPL: READ-EVAL-PRINT LOOP

        // To properly implement REPL: I have to implement a loop.

        let mut parts = trimmed.split_whitespace();
        let command = parts.next().unwrap_or("");

        match command {
            "exit 0" => std::process::exit(0),
            "echo" => {
                let args: Vec<&str> = parts.collect();
                println!("{}", args.join(" "))
            }
            _ => {
                println!("{}: command not found", command)
            }
        }
    }
}
