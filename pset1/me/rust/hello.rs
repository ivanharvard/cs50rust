use std::io;
use std::io::Write;

#[no_mangle]
pub extern "C" fn main_rs() {
    let mut name = String::new();

    print!("What's your name? ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    println!("hello, {}", name.trim());
}