use std::env;
use std::process;
use std::io::Write;

mod cs50;

const ALPHASIZE: u32 = 26;

fn print_usage() {
    println!("Usage: ./caesar key")
}

fn encode(c: char, key: u32) -> char {
    let base: u32;
    if c.is_ascii_lowercase() {
        base = b'a' as u32;
    } else if c.is_ascii_uppercase() {
        base = b'A' as u32;
    } else {
        return c;
    }

    let offset = c as u32 - base;
    let rotated = (offset + key) % ALPHASIZE;
    return char::from_u32(base + rotated).unwrap();
}

#[no_mangle]
pub extern "C" fn main_rs() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() != 2 {
        print_usage();
        process::exit(1);
    }

    let key: u32 = match argv[1].parse() {
        Ok(v)  => v,
        Err(_) => {
            print_usage();
            process::exit(1);
        }
    };

    let plaintext = cs50::get_string("plaintext:  ");
    print!("ciphertext: ");
    for c in plaintext.chars() {
        print!("{}", encode(c, key));
    }
    std::io::stdout().flush().unwrap();
    println!();
    process::exit(0);
}
