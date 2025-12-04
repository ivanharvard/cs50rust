#[allow(dead_code)]
mod cs50;

static POINTS: [i8; 26] = [1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10];

fn compute_score(word: &str) -> i8 {
    let mut score = 0;

    for c in word.chars() {
        if c.is_alphabetic() {
            let idx = (c.to_ascii_lowercase() as u8 - b'a') as usize;
            score += POINTS[idx];        
        }
    }

    return score;
}

#[no_mangle]
pub extern "C" fn main_rs() {
    let player1 = cs50::get_string("Player 1: ");
    let player2 = cs50::get_string("Player 2: ");

    let score1  = compute_score(&player1);
    let score2  = compute_score(&player2);

    if score1 > score2 {
        println!("Player 1 wins!");
    } else if score2 > score1 {
        println!("Player 2 wins!");
    } else {
        println!("Tie!")
    }
}