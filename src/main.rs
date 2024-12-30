use std::io::{self, Write};
use std::{
    env::{self},
    fs,
    path::{Path},
    process::{Command, ExitCode},
};

fn main() {
    loop {
        // Print the prompt
        print!("$ ");
        io::stdout().flush().unwrap(); // Ensure the prompt is printed before waiting for input

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();
        let mut parts = trimmed.split_whitespace();
        let command = parts.next().unwrap_or("");

        // Handle the command
        match command {
            "echo" => handle_echo(parts),
            "exit" => std::process::exit(0),
            "type" => handle_type(parts),
            _ => {
                if let Some(_path) = find_command_in_path(command) {
                    // Execute the external program with arguments
                    execute_external_command(command, parts);
                } else {
                    println!("{}: command not found", command);
                }
            }
        }
    }
}

fn handle_echo(parts: std::str::SplitWhitespace) {
    let args: Vec<&str> = parts.collect();
    println!("{}", args.join(" "));
}

fn handle_type(mut parts: std::str::SplitWhitespace) {
    let command_to_check = parts.next().unwrap_or("");

    match command_to_check {
        "exit" | "echo" | "type" => {
            println!("{} is a shell builtin", command_to_check);
        }
        _ => {
            if let Some(path) = find_command_in_path(command_to_check) {
                println!("{} is  {}", command_to_check, path.display());
            } else {
                println!("{}: not found", command_to_check);
            }
        }
        "pwd" => {
            let path = env::current_dir().unwrap();
            println!("{}", path.display());
        }
    }
}

/// Find the command in the directories listed in the `PATH` environment variable.
fn find_command_in_path(command: &str) -> Option<std::path::PathBuf> {
    let path = env::var("PATH").unwrap_or_else(|_| String::new());
    let dirs = path.split(':');
    for dir in dirs {
        let full_path = Path::new(dir).join(command);
        if full_path.is_file() {
            return Some(full_path);
        }
    }
    None
}

/// Execute an external program with arguments.
fn execute_external_command(command: &str, args: std::str::SplitWhitespace) {
    let command_path = find_command_in_path(command).unwrap();
    let output = Command::new(command_path)
        .args(args)
        .output()
        .expect("Failed to execute command");

    if !output.stdout.is_empty() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }

    if !output.stderr.is_empty() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }

    if !output.status.success() {
        std::process::exit(output.status.code().unwrap_or(1));
    }
}
