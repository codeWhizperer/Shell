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




// fn parse_command(input: &str) -> Vec<String> {
//     let mut args = Vec::new();
//     let mut current_arg = String::new();
//     let mut in_single_quote = false;
//     let mut in_double_quote = false;
//     let mut in_escape = false; // Flag for handling escape sequences
//     let mut chars = input.chars().peekable();

//     while let Some(c) = chars.next() {
//         match c {
//             '\'' if !in_double_quote => {
//                 // Handle single quotes
//                 if in_single_quote {
//                     // Closing single quote
//                     current_arg.push('\'');
//                 }
//                 in_single_quote = !in_single_quote;
//             }
//             '"' if !in_single_quote => {
//                 // Handle double quotes
//                 if in_double_quote {
//                     // Closing double quote
//                     current_arg.push('\"');
//                 }
//                 in_double_quote = !in_double_quote;
//             }
//             '\\' if !in_single_quote && !in_double_quote => {
//                 // Handle backslashes escaping characters
//                 if let Some(&next_char) = chars.peek() {
//                     if next_char == ' ' {
//                         // If followed by space, treat it as space instead of backslash
//                         current_arg.push(' ');
//                         chars.next(); // Consume space
//                         continue;
//                     }
//                 }
//                 // Treat backslash as escape character for other cases
//                 in_escape = true;
//             }
//             ' ' if !in_single_quote && !in_double_quote && !in_escape => {
//                 // Split on space if not inside quotes
//                 if !current_arg.is_empty() {
//                     args.push(current_arg.clone());
//                     current_arg.clear();
//                 }
//             }
//             _ => {
//                 // Add other characters to current argument
//                 if in_escape {
//                     current_arg.push(c); // Literal addition of escaped character
//                     in_escape = false; // Reset escape
//                 } else {
//                     current_arg.push(c);
//                 }
//             }
//         }
//     }

//     // Add the last argument if any
//     if !current_arg.is_empty() {
//         args.push(current_arg);
//     }

//     args
// }


fn parse_command(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut skip_space = false; // Flag to handle escaped spaces
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                // Handle escape sequences for backslashes and spaces
                if let Some(&next_char) = chars.peek() {
                    match next_char {
                        ' ' => {
                            // Skip the backslash and treat it as a single space
                            current_arg.push(' ');
                            chars.next(); // Consume the space
                        }
                        _ => {
                            // Treat backslash as a literal character
                            current_arg.push('\\');
                        }
                    }
                }
            }
            '"' if !in_single_quote => {
                // Toggle state for double quotes
                if in_double_quote {
                    current_arg.push('"');
                }
                in_double_quote = !in_double_quote;
            }
            '\'' if !in_double_quote => {
                // Toggle state for single quotes
                if in_single_quote {
                    current_arg.push('\'');
                }
                in_single_quote = !in_single_quote;
            }
            ' ' if !in_single_quote && !in_double_quote => {
                // Handle space outside quotes (argument boundary)
                if !current_arg.is_empty() {
                    args.push(current_arg.clone());
                    current_arg.clear();
                }
            }
            _ => {
                // Add any other characters to the current argument
                current_arg.push(c);
            }
        }
    }

    // Add the last argument if any
    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    args
}

