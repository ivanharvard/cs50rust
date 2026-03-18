use std::os::raw::{c_char, c_uint};
use std::io::BufRead;

mod cs50;

const N: usize = 1 << 17;

struct Dictionary {
    buckets: Vec<Vec<Box<[u8]>>>,
    word_count: usize,
}

impl Dictionary {
    fn new() -> Self {
        Self {
            buckets: vec![Vec::new(); N],
            word_count: 0,
        }
    }

    #[inline]
    fn hash(word: &str) -> usize {
        let mut hash: u32 = 2166136261;

        for b in word.bytes() {
            let c = b.to_ascii_lowercase();
            hash ^= c as u32;
            hash = hash.wrapping_mul(16777619);
        }

        (hash as usize) & (N - 1)
    }

    #[inline]
    fn insert(&mut self, word: String) {
        let index = Self::hash(&word);
        self.buckets[index].push(word.into_boxed_str().into_boxed_bytes());
        self.word_count += 1;
    }

    #[inline]
    fn contains(&self, word: &str) -> bool {
        let index = Self::hash(word);
        self.buckets[index]
            .iter()
            .any(|w| w.eq_ignore_ascii_case(word.as_bytes()))
    }

    #[inline]
    fn size(&self) -> usize {
        self.word_count
    }
}

static mut DICT: Option<Dictionary> = None;

#[no_mangle]
pub unsafe extern "C" fn check_rs(word: *const c_char) -> bool {
    let binding = cs50::CStringRef(word);
    let word_str = binding.to_str();
    return check(word_str);
}

fn check(word: &str) -> bool {
    unsafe {
        match DICT.as_ref() {
            Some(d) => d.contains(word),
            None => false,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn hash_rs(word: *const c_char) -> c_uint {
    let binding = cs50::CStringRef(word);
    let word_str = binding.to_str();
    return hash(word_str) as c_uint;
}

fn hash(word: &str) -> u32 {
    Dictionary::hash(word) as u32
}

#[no_mangle]
pub unsafe extern "C" fn load_rs(dictionary: *const c_char) -> bool {
    let binding = cs50::CStringRef(dictionary);
    let dictionary_str = binding.to_str();
    return load(dictionary_str);
}

fn load(dictionary: &str) -> bool {
    let mut d = Dictionary::new();

    let file = match std::fs::File::open(dictionary) {
        Ok(f) => f,
        Err(_) => return false,
    };

    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(word) => d.insert(word.to_ascii_lowercase()),
            Err(_) => return false,
        }
    }

    unsafe { DICT = Some(d) };
    true
}

#[no_mangle]
pub unsafe extern "C" fn size_rs() -> c_uint {
    size() as c_uint
}

fn size() -> u32 {
    unsafe {
        match DICT.as_ref() {
            Some(d) => d.size() as u32,
            None => 0,
        }
    }
   
}

#[no_mangle]
pub unsafe extern "C" fn unload_rs() -> bool {
    unload()
}

fn unload() -> bool {
    unsafe { DICT = None };
    true
}