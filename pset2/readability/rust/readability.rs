#[allow(dead_code)]
mod cs50;

fn calculate_stats(text: &str) -> (f32, f32) {
    let mut words: i32     = 0;
    let mut letters: i32   = 0;
    let mut sentences: i32 = 0;
    let mut in_word = false;
    for c in text.chars() {
        if c.is_alphabetic() {
            letters += 1;
            in_word = true;
        }

        if c == '.' || c == '!' || c == '?' {
            sentences += 1;
        }
        
        if c.is_whitespace() {
            if in_word {
                words += 1;
                in_word = false;
            }
        } else {
            in_word = true;
        }
    }

    if in_word {
        words += 1;
    }

    return (
        (letters as f32 / words as f32) * 100.0, 
        (sentences as f32 / words as f32) * 100.0,
    );
}

#[no_mangle]
pub extern "C" fn main_rs() {
    let text  = cs50::get_string("Text: ");

    let (l, s) = calculate_stats(&text);
    let index  = (0.0588 * l - 0.296 * s - 15.8).round() as i32;

    if index < 1 {
        println!("Before Grade 1");
    } else if index >= 16 {
        println!("Grade 16+");
    } else {
        println!("Grade {}", index);
    }
}