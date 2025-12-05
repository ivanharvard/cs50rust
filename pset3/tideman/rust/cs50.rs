#![allow(dead_code)]

use std::io;
use std::io::Write;
use std::str::FromStr;

#[allow(non_camel_case_types)]
pub type c_string = *const std::os::raw::c_char;

pub fn get_string(s: &str) -> String {
    print!("{}",s);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .unwrap();

    return input.trim().to_string();
}

pub fn get_int<T>(s: &str) -> T 
where
    T: FromStr,
{
    loop {
        // if you could successfully parse the string to type T and store it in v
        if let Ok(v) = get_string(s).parse::<T>() {
            return v;
        }
        // else loop
    }
}

pub struct CStringRef(pub *const std::os::raw::c_char);
impl CStringRef {
    pub fn to_str(&self) -> &str {
        if self.0.is_null() {
            return "";
        }

        unsafe { std::ffi::CStr::from_ptr(self.0).to_str().unwrap_or("")}
    }
}

pub unsafe fn c_arr_to_slice<'a, T>(arr: *mut T, len: std::os::raw::c_int) -> &'a mut [T] {
    return std::slice::from_raw_parts_mut(arr, len as usize);
}
