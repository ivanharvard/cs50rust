use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

mod cs50;

const MAX_VOTERS: usize = 100;
const MAX_CANDIDATES: usize = 9;

#[repr(C)]
pub struct Candidate {
    pub name: *const c_char,
    pub votes: c_int,
    pub eliminated: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn vote_rs(
    voter: c_int,
    rank: c_int,
    name: *const c_char,
    candidates: *mut Candidate,
    candidate_count: c_int,
    preferences: *mut [c_int; MAX_CANDIDATES], 
) -> bool {
    let name_str = CStr::from_ptr(name).to_str().unwrap();
    let candidates_slice = cs50::c_arr_to_slice(candidates, candidate_count);
    let preferences_rows = cs50::c_arr_to_slice(preferences, MAX_VOTERS.try_into().unwrap());

    return vote_rs_aux(
        voter as usize, 
        rank as usize, 
        name_str, 
        candidates_slice,
        preferences_rows,
    );
}

fn vote_rs_aux(
    voter: usize,
    rank: usize,
    name: &str,
    candidates: &mut [Candidate],
    preferences: &mut [[c_int; MAX_CANDIDATES]],
) -> bool {
    for (i, candidate) in candidates.iter_mut().enumerate() {
        let candidate_name = unsafe {
            CStr::from_ptr(candidate.name).to_str().unwrap()
        };

        if candidate_name == name {
            preferences[voter][rank] = i as i32;
            return true;
        }
    }
    return false;
}


#[no_mangle]
pub unsafe extern "C" fn tabulate_rs(
    candidates: *mut Candidate,
    candidate_count: c_int,
    preferences: *mut [c_int; MAX_CANDIDATES], 
    voter_count: c_int,
) {
    let candidates_slice = cs50::c_arr_to_slice(candidates, candidate_count);
    let preferences_rows = cs50::c_arr_to_slice(preferences, voter_count);

    tabulate_rs_aux(
        candidates_slice,
        preferences_rows,
    );
}

fn tabulate_rs_aux(
    candidates: &mut [Candidate],
    preferences: &mut [[c_int; MAX_CANDIDATES]],
) {
    for c in candidates.iter_mut() {
        c.votes = 0;
    }

    let voter_count = preferences.len();
    
    for voter in 0..voter_count {
        for rank in 0..MAX_CANDIDATES {
            let candidate_index = preferences[voter][rank] as usize;

            if candidate_index >= candidates.len() {
                continue;
            }

            let candidate = &mut candidates[candidate_index];

            if candidate.eliminated == 0 {
                candidate.votes += 1;
                break;
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn print_winner_rs(
    candidates: *mut Candidate,
    candidate_count: c_int,
    voter_count: c_int,
) -> c_int {
    let candidates_slice = cs50::c_arr_to_slice(candidates, candidate_count);
    return print_winner_aux(candidates_slice, voter_count as i32) as c_int;
}

fn print_winner_aux(candidates: &[Candidate], voter_count: i32) -> bool {
    for candidate in candidates {
        if candidate.eliminated == 0 && candidate.votes > voter_count / 2 {
            let name = unsafe {
                CStr::from_ptr(candidate.name)
                    .to_str()
                    .unwrap()
            };
            println!("{name}");
            return true;
        }
    }
    return false;
}

#[no_mangle]
pub unsafe extern "C" fn find_min_rs(
    candidates: *mut Candidate,
    candidate_count: c_int,
) -> c_int {
    let candidates_slice = cs50::c_arr_to_slice(candidates, candidate_count);
    return find_min_aux(candidates_slice) as c_int;
}

fn find_min_aux(candidates: &mut [Candidate]) -> i32 {
    let mut min: i32 = i32::MAX;
    for candidate in candidates.iter_mut() {
        if candidate.eliminated == 0 && candidate.votes < min {
            min = candidate.votes;
        }
    }
    return min;
}

#[no_mangle]
pub unsafe extern "C" fn is_tie_rs(
    min: c_int,
    candidates: *mut Candidate,
    candidate_count: c_int,
) -> c_int {
    let candidates_slice = cs50::c_arr_to_slice(candidates, candidate_count);
    return is_tie_rs_aux(min, candidates_slice) as c_int;
}

fn is_tie_rs_aux(min: c_int, candidates: &mut [Candidate]) -> i32 {
    for candidate in candidates.iter_mut() {
        if candidate.eliminated == 0 && candidate.votes != min {
            return 0;
        }
    }
    return 1;
}

#[no_mangle]
pub unsafe extern "C" fn eliminate_rs(
    min: c_int,
    candidates: *mut Candidate,
    candidate_count: c_int,
) {
    let candidates_slice = cs50::c_arr_to_slice(candidates, candidate_count);
    eliminate_rs_aux(
        min as i32,
        candidates_slice,
    );
}

fn eliminate_rs_aux(
    min: i32,
    candidates: &mut [Candidate],
) {
    for candidate in candidates.iter_mut() {
        if candidate.votes == min {
            candidate.eliminated = 1;
        }
    }
}