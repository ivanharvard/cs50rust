use std::io;
use std::io::Write;

fn get_string(s: &'static str) -> String {
    print!("{}",s);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .unwrap();

    return input.trim().to_string();
}

fn get_i32(s: &'static str) -> i32 {
    loop {
        let s = get_string(s);
        let int = match s.parse::<i32>() {
            Ok(v)  => v,
            Err(_) => continue,
        };
        return int;
    }
}

fn print_chars(ch: char, n: i32) {
    for _ in 0..n {
        print!("{}", ch);
    }
    io::stdout().flush().unwrap();
}

#[no_mangle]
pub extern "C" fn main_rs() {
    let mut height: i32 = -1;
    while !(1 <= height && height <= 8) {
        height = get_i32("Height: ");
    }

    for i in 1..=height {
        print_chars(' ', height - i);
        print_chars('#', i);
        print!("  ");
        io::stdout().flush().unwrap();
        print_chars('#', i);
        println!();
    }

    
}