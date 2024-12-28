#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    env::{self, args},
    fs,
    os::unix::process,
    path::{self, Path},
    process::{Command, ExitCode},
};

// fn main() {
//     loop {
//         // Uncomment this block to pass the first stage
//         print!("$ ");
//         io::stdout().flush().unwrap();

//         // Wait for user input
//         let stdin = io::stdin();
//         let mut input = String::new();
//         stdin.read_line(&mut input).unwrap();
//         // let command = input.trim();
//         let trimmed = input.trim();

//         // REPL: READ-EVAL-PRINT LOOP

//         // To properly implement REPL: I have to implement a loop.

//         let mut parts = trimmed.split_whitespace();
//         let command = parts.next().unwrap_or("");

//         match command {
//             "echo" => {
//                 let args: Vec<&str> = parts.collect();
//                 println!("{}", args.join(" "))
//             }
//             "exit" => std::process::exit(0),
//             "type" => {
//                 let command_to_check = parts.next().unwrap_or("");
//                 match command_to_check {
//                     "exit" | "echo" | "type" => {
//                         println!("{} is a shell builtin", command_to_check)
//                     }
//                     _ => {
//                         let path = env::var("PATH").unwrap_or_default();
//                         let dirs = path.split(':');
//                         let mut found = false;

//                         for dir in dirs {
//                             let full_path = Path::new(dir).join(command_to_check);
//                             if full_path.exists() && fs::metadata(&full_path).unwrap().is_file() {
//                                 println!(
//                                     "{} is {}",
//                                     command_to_check,
//                                     full_path.display()
//                                 );
//                                 found = true;
//                                 break;
//                             }
//                         }
//                         if !found {
//                             println!("{}: not found", command_to_check)
//                         }
//                     }
//                 }
//             }
//             _ => {
//                 println!("{}: command not found", command)
//             }
//         }
//     }
// }


fn main (){
    loop{
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();
        let mut parts = trimmed.split_whitespace();
        let command = parts.next().unwrap_or("");
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



fn handle_echo(parts:std::str::SplitWhitespace){
    let args: Vec<&str> = parts.collect();
     println!("{}", args.join(" "))
}

fn handle_type(mut parts: std::str::SplitWhitespace) {
    let command_to_check = parts.next().unwrap_or("");

    match command_to_check {
        "exit" | "echo" | "type" => {
            println!("{} is a shell builtin", command_to_check);
        }
        _ => {
            if let Some(path) = find_command_in_path(command_to_check) {
                println!("{} is located at: {}", command_to_check, path.display());
            } else {
                println!("{}: not found", command_to_check);
            }
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


fn execute_external_command(command: &str, args: std::str::SplitWhitespace) {
    let path = find_command_in_path(command).expect("Command not found");
    let output = Command::new(path)
        .args(args)
        .output()
        .expect("Failed to execute command");

    if !output.stdout.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
}