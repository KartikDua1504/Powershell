#[allow(unused_imports)]
use std::io::{self, Write};
fn main() {
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        print!("$ ");
        stdout.flush().unwrap();
        if stdin.read_line(&mut input).is_err() {
            break; 
        }
        let trimmed = input.trim();
        if trimmed.is_empty() {
            input.clear();
            continue;
        }
        if trimmed == "exit 0" {
            break;
        }
        println!("{}: command not found", trimmed);
        input.clear();
    }
}
