use std::env;
use std::io::{self, Write};
#[allow(unused_imports)]
use std::process::Command;
const BUILTINS: [&str; 3] = ["exit", "echo", "type"];
fn path(command: &str) -> Option<String> {
    let path = std::env::var("PATH").unwrap();
    for dir in path.split(':') {
        for item in std::fs::read_dir(dir).unwrap() {
            let entry = item.unwrap();
            let name = entry.file_name();
            if name == command {
                return Some(entry.path().into_os_string().into_string().unwrap());
            }
        }
    }
    None
}
fn execute(command: &str, args: &[String]) {
    let command_path = path(command);
    if command_path.is_none() {
        println!("{command}: not found");
        return;
    }
    let output = Command::new(command)
        .args(args)
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
}
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        // Handle missing commands
        let args = input
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if args.is_empty() {
            println!();
            continue;
        }
        let command = &args[0];
        let arg_string = &args[1..].join(" ");
        match &command[..] {
            "type" => {
                if BUILTINS.iter().any(|&b| b == arg_string) {
                    println!("{arg_string} is a shell builtin");
                } else if let Some(path) = path(arg_string) {
                    println!("{arg_string} is {path}")
                } else {
                    println!("{arg_string}: not found");
                }
            }
            "echo" => println!("{arg_string}"),
            "exit" => break,
            "pwd" => println!("{}", env::current_dir().unwrap().display()),
            _ => execute(command, &args[1..]),
        }
    }
}