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
        let input_trimmed = input.trim_end(); 
        if input_trimmed.is_empty() {
            continue; 
        }
        println!("{}: command not found", input_trimmed);
        input.clear();
    }
}
