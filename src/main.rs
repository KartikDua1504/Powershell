#[allow(unused_imports)]
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
        let trimmed = input.trim().to_owned();
        input.clear(); 
        if trimmed.is_empty() {
            continue;
        }
        match trimmed.split_once(' ') {
            Some((command, args)) => match command {
                "exit" if args == "0" => break,
                "echo" => println!("{}", args),
                "type" => match args {
                    "echo" | "exit" | "type" => println!("{} is a shell builtin", args),
                    _ => println!("{}: not found", args),
                },
                _ => println!("{}: command not found", trimmed),
            },
            None => println!("{}: command not found", trimmed),
        }
    }
}
