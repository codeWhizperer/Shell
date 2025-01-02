use dirs;
use std::io::{self, Write};
use std::{
    env::{self},
    path::Path,
    process::Command,
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
        // parts ["echo", "Hello", "world!!"]
        let parts = parse_command(trimmed);
        if parts.is_empty() {
            continue;
        }
        // e.g echo Hello World !!! ==> echo is the command
        let command = &parts[0];
        // e.g args ==> ["Hello", "World", "!!!"]
        let args = parts[0..].to_vec();

        // Handle the command
        match command.as_str() {
            "echo" => handle_echo(args),
            "exit" => std::process::exit(0),
            "type" => handle_type(args),
            "pwd" => match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => eprintln!("Error getting current directory: {}", e),
            },
            "cd" => {
                if let Some(directory) = args.get(0) {
                    // Home directory
                    let new_path = if directory.starts_with("~") {
                        dirs::home_dir().unwrap_or_else(|| {
                            eprintln!("cd: Unable to find the home directory");
                            std::process::exit(1);
                        })
                    } else if directory.starts_with("/") {
                        // Absolute path: Start directly from the provided directory
                        Path::new(directory).to_path_buf()
                    } else {
                        // Relative path: Start from the current directory
                        let mut current_path = env::current_dir().unwrap();
                        for segment in directory.split('/') {
                            match segment {
                                "" | "." => continue, // Skip empty and current directory components
                                ".." => {
                                    current_path.pop();
                                } // Move up one directory
                                _ => current_path.push(segment), // Add new directory segment
                            }
                        }
                        current_path
                    };

                    // Attempt to change the directory
                    if env::set_current_dir(&new_path).is_err() {
                        eprintln!("cd: {}: No such file or directory", directory);
                    }
                } else {
                    // No directory argument provided
                    eprintln!("cd: No such file or directory");
                }
            }
            _ => {
                if let Some(_path) = find_command_in_path(command) {
                    // Execute the external program with arguments
                    execute_external_command(command, &args);
                } else {
                    println!("{}: command not found", command);
                }
            }
        }
    }
}

fn handle_echo(args: Vec<String>) {
    println!("{}", args[1..].join(" "));
}

fn handle_type(args: Vec<String>) {
    if let Some(command_to_check) = args.get(0) {
        match command_to_check.as_str() {
            "exit" | "echo" | "type" | "pwd" | "cd" => {
                println!("{} is a shell builtin", command_to_check);
            }
            _ => {
                if let Some(path) = find_command_in_path(command_to_check) {
                    println!("{} is {}", command_to_check, path.display());
                } else {
                    println!("{}: not found", command_to_check);
                }
            }
        }
    } else {
        eprintln!("type: missing argument");
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
fn execute_external_command(command: &str, args: &[String]) {
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

// fn parse_command(input: &str) -> Vec<String> {
//     let mut args = Vec::new();
//     let mut current_arg = String::new();
//     let mut in_single_quote = false;
//     let mut in_double_quote = false;

//     for c in input.chars() {
//         match c {
//             '\'' => in_single_quote = !in_single_quote,
//             ' ' | '\t' if !in_single_quote && !in_double_quote => {
//                 if !current_arg.is_empty() {
//                     args.push(current_arg.clone());
//                     current_arg.clear();
//                 }
//             }

//             '\"' => in_double_quote = !in_double_quote,
//             // ' ' if !in_double_quote => {
//             //     if !current_arg.is_empty() {
//             //         args.push(current_arg.clone());
//             //         current_arg.clear();
//             //     }
//             // }
//             _ => {
//                 current_arg.push(c);
//             }
//         }
//     }

//     if !current_arg.is_empty() {
//         args.push(current_arg);
//     }

//     if in_single_quote {
//         eprintln!("Error: Unmatched single quote");
//     }
//     if in_double_quote{
//         eprintln!("Error: Unmatched double quote");

//     }

//     args
// }

fn parse_command(input: &str) -> Vec<String>{
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;

    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            // Handle single quote
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
            }

            // Handle double quote
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
            }

            // Handle whitespace outside quotes
            ' ' | '\t' if !in_single_quote && !in_double_quote => {
                if !current_arg.is_empty() {
                    args.push(current_arg.clone());
                    current_arg.clear();
                }
            }

            // Handle characters inside quotes or regular text
            _ => {
                current_arg.push(c);
            }
        }
    }

    // Add the last argument if it exists
    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    args 
}