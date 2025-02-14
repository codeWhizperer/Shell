use dirs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
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
        let args = parts[1..].to_vec();

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
                    execute_external_command(command, args);
                } else {
                    println!("{}: command not found", command);
                }
            }
        }
    }
}

fn handle_echo(args: Vec<String>) {
    // Join all arguments into a single string separated by space and print
    println!("{}", args.join(" "));
}

fn handle_type(args: Vec<String>) {
    if let Some(command_to_check) = args.get(0) {
        if command_to_check == "type" {
            // Special case: "type" is a shell builtin
            println!("type is a shell builtin");
        } else {
            // Check for other shell builtins
            match command_to_check.as_str() {
                "exit" | "echo" | "pwd" | "cd" => {
                    println!("{} is a shell builtin", command_to_check);
                }
                _ => {
                    // If not a shell builtin, check in PATH for the command
                    if let Some(path) = find_command_in_path(command_to_check) {
                        println!("{} is {}", command_to_check, path.display());
                    } else {
                        println!("{}: not found", command_to_check);
                    }
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
        if full_path.is_file()
            && full_path
                .metadata()
                .map_or(false, |m| m.permissions().mode() & 0o111 != 0)
        {
            return Some(full_path);
        }
    }
    None
}
/// Execute an external program with arguments.
fn execute_external_command(command: &str, args: Vec<String>) {
    let command_path = find_command_in_path(command).expect("Command not found");

    let output = Command::new(command_path)
        .args(&args) // Pass the arguments vector directly
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


fn parse_command(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\\' => handle_backslash(&mut chars, &mut current_arg), // Handle backslashes
            '"' => handle_double_quote(c, &mut in_double_quote, &mut current_arg), // Handle double quotes
            '\'' => handle_single_quote(c, &mut in_single_quote, &mut current_arg), // Handle single quotes
            ' ' => handle_space(&mut in_single_quote, &mut in_double_quote, &mut current_arg, &mut args), // Handle spaces between arguments
            _ => current_arg.push(c), // Add regular characters to current argument
        }
    }

    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    args
}

fn handle_backslash(chars: &mut std::iter::Peekable<std::str::Chars>, current_arg: &mut String) {
    if let Some(&next_char) = chars.peek() {
        match next_char {
            ' ' => {
                current_arg.push(' '); // Handle space after backslash
                chars.next(); // Consume the space
            }
            '"' => {
                current_arg.push('"'); // Handle double quote after backslash
                chars.next(); // Consume the double quote
            }
            '\'' => {
                current_arg.push('\''); // Handle single quote after backslash
                chars.next(); // Consume the single quote
            }
            '\\' => {
                current_arg.push('\\'); // Handle backslash after backslash
                chars.next(); // Consume the backslash
            }
            'n' => {
                current_arg.push('\n'); // Replace '\n' with a newline character
                chars.next(); // Consume the 'n'
            }
            't' => {
                current_arg.push('\t'); // Replace '\t' with a tab character
                chars.next(); // Consume the 't'
            }
            _ => {
                current_arg.push('\\'); // Default behavior: keep the backslash
            }
        }
    }
}

fn handle_double_quote(c: char, in_double_quote: &mut bool, current_arg: &mut String) {
    if *in_double_quote {
        return;
    }
    *in_double_quote = !*in_double_quote;
}

fn handle_single_quote(c: char, in_single_quote: &mut bool, current_arg: &mut String) {
    if *in_single_quote {
        return;
    }
    *in_single_quote = !*in_single_quote; 
}

fn handle_space(in_single_quote: &mut bool, in_double_quote: &mut bool, current_arg: &mut String, args: &mut Vec<String>) {
    if !*in_single_quote && !*in_double_quote && !current_arg.is_empty() {
        args.push(current_arg.clone()); 
        current_arg.clear(); 
    } else if *in_single_quote || *in_double_quote {
        current_arg.push(' '); 
    }
}
