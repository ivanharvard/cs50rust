use std::io::Write;
use std::process;
use std::env;

mod cs50;

const ALPHASIZE: u32 = 26;

fn encode(c: char, key: &Vec<char>) -> char { 
    if c.is_ascii_lowercase() {
        let idx = (c as u8 - b'a') as usize;
        return key[idx].to_ascii_lowercase();
    } else if c.is_ascii_uppercase() {
        let idx = (c as u8 - b'A') as usize;
        return key[idx]
    } else {
        return c;
    }
}

#[no_mangle]
pub extern "C" fn main_rs() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() != 2 {
        println!("Usage: ./caesar key");
        process::exit(1);
    }

    let mut key: Vec<char> = argv[1].chars().collect();
    if key.len() != ALPHASIZE as usize {
        println!("Key must contain {} characters.", ALPHASIZE);
        process::exit(1);
    }

    let mut seen: [bool; ALPHASIZE as usize] = [false; ALPHASIZE as usize];
    for c in key.iter_mut() {
        if !c.is_ascii_alphabetic() {
            println!("Key must only contain alphabetic characters.");
            process::exit(1);
        }

        *c = c.to_ascii_uppercase();
        let idx = (*c as u8 - b'A') as usize;
        if seen[idx] {
            println!("Key must not contain repeated characters.");
            process::exit(1);
        } else {
            seen[idx] = true;
        }
    }

    let plaintext = cs50::get_string("plaintext:  ");
    print!("ciphertext: ");
    for c in plaintext.chars() {
        print!("{}", encode(c, &key));
    }
    std::io::stdout().flush().unwrap();
    println!();
    process::exit(0);
}

