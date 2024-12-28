#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    env::{self, args},
    fs,
    os::unix::process,
    path::Path,
    process::ExitCode,
};

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
            "echo" => {
                let args: Vec<&str> = parts.collect();
                println!("{}", args.join(" "))
            }
            "exit" => std::process::exit(0),
            "type" => {
                let command_to_check = parts.next().unwrap_or("");
                match command_to_check {
                    "exit" | "echo" | "type" => {
                        println!("{} is a shell builtin", command_to_check)
                    }
                    _ => {
                        let path = env::var("PATH").unwrap_or_default();
                        let dirs = path.split(':');
                        let mut found = false;

                        for dir in dirs {
                            let full_path = Path::new(dir).join(command_to_check);
                            if full_path.exists() && fs::metadata(&full_path).unwrap().is_file() {
                                println!(
                                    "{} is {}",
                                    command_to_check,
                                    full_path.display()
                                );
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            println!("{}: not found", command_to_check)
                        }
                    }
                }
            }
            _ => {
                println!("{}: command not found", command)
            }
        }
    }
}
