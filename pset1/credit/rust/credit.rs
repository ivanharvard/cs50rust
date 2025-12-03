
mod cs50;

fn luhns_algorithm(mut ccn: i64) -> (bool, i32) {
    let mut step1 = 0;
    let mut step2 = 0;
    let mut i = 0;

    // every other digit starting from second-to-last
    while ccn > 0 {
        let mut digit = (ccn % 10) as i32;     // extract the digit
        ccn /= 10;                             // get rid of that digit

        if i % 2 == 0 {
            step2 += digit;                    // sum of the digits not involved in step1
        } else {
            digit *= 2;                        // mult by 2
            step1 += digit % 10 + digit / 10;  // add digits (12 => 1 + 2)
        }
        
        i += 1;
    }

    return ((step1 + step2) % 10 == 0, i); // step 3, returns length as well
}

#[no_mangle]
pub extern "C" fn main_rs() {
    let ccn: i64 = cs50::get_int("Number: ");
    let (passed, length) = luhns_algorithm(ccn);

    if !passed {
        println!("INVALID");
        return;
    }

    let first_two = ccn / 10_i64.pow((length - 2) as u32);
    let first     = first_two / 10;

    if length == 15 && (first_two == 34 || first_two == 37) {
        println!("AMEX");
    } else if length == 16 && (51..56).contains(&first_two) {
        println!("MASTERCARD");
    } else if (length == 13 || length == 16) && first == 4 {
        println!("VISA");
    } else {
        println!("INVALID");
    }
}