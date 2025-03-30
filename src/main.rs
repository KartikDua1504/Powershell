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
                if let Some(path) = find_executable_in_path(cmd) {
                    let status = process::Command::new(path)
                        .args(args)
                        .spawn()
                        .and_then(|mut child| child.wait());

                    match status {
                        Ok(exit_status) => {
                            if !exit_status.success() {
                                eprintln!("{} exited with status {}", cmd, exit_status);
                            }
                        }
                        Err(err) => eprintln!("Failed to execute {}: {}", cmd, err),
                    }
                } else {
                    println!("{}: command not found", cmd);
                }
            }
            _ => println!("{}: command not found", trimmed),
        }
    }
}
