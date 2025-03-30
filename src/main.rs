mod builtins;
use crate::builtins::COMMANDS;
use pathsearch::find_executable_in_path;
use std::io::{self, Write};
use std::process;

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    loop {
        print!("$ ");
        stdout.flush().unwrap();

        if stdin.read_line(&mut input).is_err() {
            break;
        }

        let trimmed = input.trim().to_owned();
        input.clear();

        if trimmed.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = trimmed.split_whitespace().collect();

        match tokens.as_slice() {
            ["exit"] => process::exit(0),
            ["exit", code] => {
                if let Ok(num) = code.parse::<i32>() {
                    process::exit(num);
                }
            }
            ["echo", ..] => println!("{}", tokens[1..].join(" ")),
            ["type", cmd] => {
                if COMMANDS.contains(cmd) {
                    println!("{} is a shell builtin", cmd);
                } else if let Some(path) = find_executable_in_path(cmd) {
                    println!("{} is {}", cmd, path.to_str().unwrap());
                } else {
                    println!("{}: not found", cmd);
                }
            }
            [cmd, args @ ..] => {
                // Check if command exists in PATH
                if find_executable_in_path(cmd).is_some() {
                    // Execute using command name instead of full path
                    match process::Command::new(cmd)
                        .args(args)
                        .output() 
                    {
                        Ok(output) => {
                            io::stdout().write_all(&output.stdout).unwrap();
                            io::stderr().write_all(&output.stderr).unwrap();
                        }
                        Err(e) => {
                            eprintln!("Failed to execute {}: {}", cmd, e);
                        }
                    }
                } else {
                    println!("{}: command not found", cmd);
                }
            }
            _ => println!("{}: command not found", trimmed),
        }
    }
}
