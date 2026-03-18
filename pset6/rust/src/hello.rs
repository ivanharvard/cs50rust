use std::io;
use std::io::Write;

pub fn main() {
    let mut name = String::new();

    print!("What's your name? ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    println!("hello, {}", name.trim());
}