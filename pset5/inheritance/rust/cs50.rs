#![allow(dead_code)]

use std::io;
use std::io::Write;
use std::str::FromStr;
use std::os::raw::c_int;

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

pub unsafe fn c_arr_to_slice<'a, T>(arr: *mut T, len: c_int) -> &'a mut [T] {
    std::slice::from_raw_parts_mut(arr, len as usize)
}

pub struct Array2D<'a, T> {
    pub width: usize,
    pub height: usize,
    pub data: &'a mut [T],
}

pub struct Array2DOwned<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<'a, T: Clone> Array2D<'a, T> {
    pub fn new(data: &'a mut [T], width: usize, height: usize) -> Self {
        assert_eq!(data.len(), width * height);
        return Self { width, height, data }
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        return &self.data[row * self.width + col];
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        return &mut self.data[row * self.width + col];
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item=&mut [T]> {
        return self.data.chunks_exact_mut(self.width);
    }

    // returns an independent, owned array
    pub fn to_owned(&self) -> Array2DOwned<T> {
        return Array2DOwned {
            width: self.width,
            height: self.height,
            data: self.data.to_vec(),
        };
    }
}

impl<T: Clone> Array2DOwned<T> {
    pub fn get(&self, row: usize, col: usize) -> &T {
        return &self.data[row * self.width + col]
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        return &mut self.data[row * self.width + col]
    }
}