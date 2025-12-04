#![allow(dead_code)]

use std::io;
use std::io::Write;
use std::str::FromStr;

pub fn get_string(s: &str) -> String {
    print!("{}",s);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .unwrap();

    return input.trim().to_string();
}

pub fn get_int<T>(s: &str) -> T 
where
    T: FromStr,
{
    loop {
        // if you could successfully parse the string to type T and store it in v
        if let Ok(v) = get_string(s).parse::<T>() {
            return v;
        }
        // else loop
    }
}