use std::os::raw::{c_char, c_int, c_uchar};
use std::ffi::CStr;
use std::ffi::CString;

mod cs50;

// Max number of candidates
const MAX: usize = 9;

// Candidates have name and vote count
#[repr(C)]
pub struct Candidate {
    pub name: *const c_char,
    pub votes: c_int,
}

#[no_mangle]
pub extern "C" fn main_rs() {
    // Check for invalid usage
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 2 {
        println!("Usage: plurality [candidate ...]");
        std::process::exit(1);
    }

    // Vector of candidates
    let mut candidates: Vec<Candidate> = Vec::with_capacity(MAX); 
    let mut candidates_names: Vec<CString> = Vec::with_capacity(MAX);

    // Number of candidates
    let candidate_count: usize = argv.len() - 1;
    if candidate_count > MAX {
        println!("Maximum number of candidates is {}", MAX);
        std::process::exit(2);
    }

    // Populate vec of candidates
    for name in argv[1..].iter() {
        let c_name = CString::new(name.as_str()).unwrap();
        let ptr    = c_name.as_ptr();

        candidates_names.push(c_name);
        candidates.push(Candidate {
            name: ptr,
            votes: 0,
        });
    }

    let voter_count: i32 = cs50::get_int("Number of voters: ");

    // Loop over all voters
    for _ in 0..voter_count {
        let name = cs50::get_string("Vote: ");
        // Check for invalid vote
        if !vote_rs_aux(&name, &mut candidates[..])
        {
            println!("Invalid vote.");
            std::process::exit(3);
        }
    }

    // Display winner of election
    print_winner_rs_aux(&candidates);
}

#[no_mangle]
pub unsafe extern "C" fn vote_rs(
    name: *const c_char,
    candidates: *mut Candidate,
    candidate_count: usize,
) -> c_uchar {
    // converting c strings to rust &str
    let c_str    = CStr::from_ptr(name);
    let name_str = c_str.to_str().unwrap_or("");
    let candidates_slice  = std::slice::from_raw_parts_mut(candidates, candidate_count);

    return vote_rs_aux(name_str, candidates_slice) as c_uchar;
}

fn vote_rs_aux(name: &str, candidates: &mut [Candidate]) -> bool {
    for candidate in candidates.iter_mut() {
        let c_name         = cs50::CStringRef(candidate.name);
        let candidate_name = c_name.to_str();

        if candidate_name == name {
            candidate.votes += 1;
            return true;
        }
    }
    return false;
}

#[no_mangle]
pub unsafe extern "C" fn print_winner_rs(
    candidates: *mut Candidate, 
    candidate_count: usize
) {
    let candidates_slice = std::slice::from_raw_parts_mut(candidates, candidate_count);
    print_winner_rs_aux(candidates_slice); 
}

fn print_winner_rs_aux(candidates: &[Candidate]) {
    let mut max = i32::MIN;
    for candidate in candidates.iter() {
        max = if candidate.votes > max { candidate.votes } 
              else { max };
    }

    for candidate in candidates.iter() {
        if candidate.votes == max {
            let c_name = cs50::CStringRef(candidate.name);
            let name   = c_name.to_str();
            println!("{}", name);
        }
    }
}