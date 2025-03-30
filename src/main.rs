mod builtins;
use crate::builtins::COMMANDS;
use pathsearch::find_executable_in_path;
use std::io::{self, Write};

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

        let trimmed = input.trim();
        input.clear();

        if trimmed.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = trimmed.split_whitespace().collect();

        match tokens.as_slice() {
            ["exit"] => std::process::exit(0),
            ["exit", code] => {
                if let Ok(num) = code.parse::<i32>() {
                    std::process::exit(num);
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
            _ => println!("{}: command not found", trimmed),
        }
    }
}
