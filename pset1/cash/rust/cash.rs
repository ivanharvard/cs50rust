mod cs50;

#[no_mangle]
pub extern "C" fn main_rs() {
    let mut cents = -1;
    while cents <= 0 {
        cents = cs50::get_i32("Change owed: ");
    }

    let mut coins = 0;
    
    coins += cents / 25;
    cents %= 25;

    coins += cents / 10;
    cents %= 10;

    coins += cents / 5;
    cents %= 5;

    println!("{}", coins + cents);
}