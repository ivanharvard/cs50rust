use std::io;
use std::io::Write;

pub fn get_string(s: &'static str) -> String {
    print!("{}",s);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .unwrap();

    return input.trim().to_string();
}

pub fn get_i32(s: &'static str) -> i32 {
    loop {
        let s = get_string(s);
        let int = match s.parse::<i32>() {
            Ok(v)  => v,
            Err(_) => continue,
        };
        return int;
    }
}