#[allow(unused_imports)]
use std::io::{self, Write};
fn main() {
    let stdin = io::stdin();
    loop{
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let stdin.read_line(&mut input).is_err(){
        break;
    }
    let input = input.trim();
    if input.is_empty(){
        continue;
    }
    println!("{}: command not found",input);
}
}
