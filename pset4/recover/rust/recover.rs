use std::io::{Read, Write};

mod cs50;

const BYTE0: u8 = 0xFF;
const BYTE1: u8 = 0xD8;
const BYTE2: u8 = 0xFF;
const BYTE3_MASK: u8 = 0xF0;
const BYTE3_HI_NIBBLE: u8 = 0xE0; 

const BLOCK_SIZE: usize = 512;

#[no_mangle]
pub extern "C" fn main_rs() { 
    let argv: Vec<String> = std::env::args().collect();

    if argv.len() != 2 {
        println!("Usage: ./recover image");
        std::process::exit(1);
    }

    let filename = &argv[1];

    let mut file = match std::fs::File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            println!("Could not open file: {}", e);
            std::process::exit(1);
        }
    };

    let mut buf = [0u8; BLOCK_SIZE];

    let mut file_count = 0;
    loop {
        let bytes_read = file.read(&mut buf).unwrap();
        if bytes_read == 0 {
            break; // EOF
        }

        if buf[0] == BYTE0 &&
           buf[1] == BYTE1 &&
           buf[2] == BYTE2 &&
           (buf[3] & BYTE3_MASK) == BYTE3_HI_NIBBLE {
            // found a new JPEG header
            let out_filename = format!("{:03}.jpg", file_count);
            let mut out = std::fs::File::create(out_filename).unwrap();
            out.write_all(&buf).unwrap();
            file_count += 1;
        }
    }


    
}