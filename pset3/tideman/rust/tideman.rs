use std::os::raw::c_int;
use std::ffi::CStr;

mod cs50;

const MAX: usize = 9;

#[repr(C)]
pub struct pair {
    winner: c_int,
    loser: c_int,
} 

#[no_mangle]
pub unsafe extern "C" fn vote_rs(
    rank: c_int,
    name: cs50::c_string,
    ranks: *mut c_int,
    candidates: *mut cs50::c_string,
    candidate_count: c_int
) -> c_int {
    let name_str = CStr::from_ptr(name).to_str().unwrap();
    let ranks_slice      = cs50::c_arr_to_slice(ranks, candidate_count);
    let candidates_slice = cs50::c_arr_to_slice(candidates, candidate_count);

    return vote(rank as usize, name_str, ranks_slice, candidates_slice) as c_int;
}

fn vote(
    rank: usize, 
    name: &str, 
    ranks: &mut [c_int], 
    candidates: &mut [cs50::c_string],
) -> bool {
    for i in 0..candidates.len() {
        let candidate = candidates[i];
        let candidate_name = unsafe {
            CStr::from_ptr(candidate)
                .to_str()
                .unwrap()
        };

        if candidate_name == name { 
            ranks[rank] = i as c_int;
            return true;
        }
    }
    return false;
}

#[no_mangle]
pub unsafe extern "C" fn record_preferences_rs(
    ranks: *mut c_int,
    preferences: *mut c_int,
    candidate_count: c_int,
) {
    let ranks_slice       = cs50::c_arr_to_slice(ranks, candidate_count);
    let preferences_slice = std::slice::from_raw_parts_mut(
        preferences as *mut c_int as *mut [c_int; MAX],
        MAX,
    );
    record_preferences(ranks_slice, preferences_slice, candidate_count as usize);
}

fn record_preferences(
    ranks: &mut [c_int],
    preferences: &mut [[c_int; MAX]],
    candidate_count: usize,
) {
    for i in 0..candidate_count {
        for j in (i+1)..candidate_count {
            let winner = ranks[i] as usize;
            let loser  = ranks[j] as usize;
            preferences[winner][loser] += 1;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn add_pairs_rs(
    preferences: *mut c_int,
    candidate_count: c_int,
    pair_count_out: *mut c_int,
    pairs: *mut pair
) {
    let preferences_slice = std::slice::from_raw_parts_mut(
        preferences as *mut c_int as *mut [c_int; MAX],
        MAX,
    );
    let pairs_slice       = cs50::c_arr_to_slice(pairs, (MAX * (MAX - 1) / 2) as i32);
    
    *pair_count_out = add_pairs(
        preferences_slice, 
        candidate_count as usize, 
        pairs_slice
    ) as c_int;
}

fn add_pairs(
    preferences: &[[c_int; MAX]],
    candidate_count: usize,
    pairs: &mut [pair],
) -> usize {
    let mut pair_count = 0;

    for i in 0..candidate_count {
        for j in (i+1)..candidate_count {
            if preferences[i][j] > preferences[j][i] {
                pairs[pair_count].winner = i as c_int;
                pairs[pair_count].loser  = j as c_int;
                pair_count += 1;
            } else if preferences[i][j] < preferences[j][i] {
                pairs[pair_count].winner = j as c_int;
                pairs[pair_count].loser  = i as c_int;
                pair_count += 1;
            }
            // skip if eq
        }
    }

    return pair_count;
}


#[no_mangle]
pub unsafe extern "C" fn sort_pairs_rs(
    pairs: *mut pair,
    pair_count: c_int,
    preferences: *mut c_int,
) {
    let pairs_slice       = cs50::c_arr_to_slice(pairs, pair_count);
    let preferences_slice = std::slice::from_raw_parts_mut(
        preferences as *mut c_int as *mut [c_int; MAX],
        MAX,
    );

    sort_pairs(pair_count as usize, pairs_slice, preferences_slice);
}

fn sort_pairs(
    pair_count: usize,
    pairs: &mut [pair],
    preferences: &[[c_int; MAX]],
) {
    pairs[..pair_count].sort_by_key(
        |p| std::cmp::Reverse(how_badly_beat(p, preferences))
    );
}

fn how_badly_beat(
    p: &pair,
    preferences: &[[c_int; MAX]],
) -> i32 {
    let winner = p.winner as usize;
    let loser  = p.loser as usize;
    return preferences[winner][loser] - preferences[loser][winner]; 
}

#[no_mangle]
pub unsafe extern "C" fn lock_pairs_rs(
    pairs: *mut pair,
    pair_count: c_int,
    candidate_count: c_int,
    locked: *mut u8,
) {
    let pairs_slice  = cs50::c_arr_to_slice(pairs, pair_count);
    let locked_slice = std::slice::from_raw_parts_mut(
        locked as *mut u8 as *mut [u8; MAX],
        MAX,
    );

    lock_pairs(pairs_slice, pair_count as i32, candidate_count, locked_slice);
}

fn lock_pairs(
    pairs: &mut [pair],
    pair_count: i32,
    candidate_count: i32,
    locked: &mut [[u8; MAX]]
) {
    for i in 0..(pair_count as usize) {
        let winner = pairs[i].winner as usize;
        let loser  = pairs[i].loser as usize;

        if !is_cycle(winner, loser, candidate_count, locked) {
            locked[winner][loser] = 1;
        }
    }
}

fn is_cycle(
    v1: usize, 
    v2: usize, 
    vertex_count: i32, 
    edges: &mut [[u8; MAX]]
) -> bool {
    // base case
    if v1 == v2 {
        return true;
    }

    for v3 in 0..vertex_count {
        if edges[v2][v3 as usize] == 1 && is_cycle(v1, v3 as usize, vertex_count, edges) {
            return true;
        }
    }
    
    return false;
}

#[no_mangle]
pub unsafe extern "C" fn print_winner_rs(
    candidates: *mut cs50::c_string,
    locked: *mut u8,
    candidate_count: c_int,
) {
    let candidates_slice = cs50::c_arr_to_slice(candidates, candidate_count as c_int);
    let locked_slice     = std::slice::from_raw_parts_mut(
        locked as *mut u8 as *mut [u8; MAX],
        MAX,
    );
    print_winner(candidates_slice, locked_slice);
}

fn print_winner(
    candidates: &mut [cs50::c_string],
    locked: &mut [[u8; MAX]],
) {
    if let Some(source) = find_source(locked, candidates.len()) {
        let name = unsafe { CStr::from_ptr(candidates[source]).to_str().unwrap() };
        println!("{name}");
    }
}

fn find_source(
    edges: &mut [[u8; MAX]],
    vertex_count: usize,
) -> Option<usize> {
    'outer: for i in 0..vertex_count {
        // if an edge exists from v_j to v_i, then there is an incoming edge
        for j in 0..vertex_count {
            if edges[j][i] != 0 {
                continue 'outer; // break
            }
        }  

        // no incoming edge, must be source
        return Some(i);
    }

    return None;
}