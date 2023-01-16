use std::io::{self, stdout, Read};

use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn die(error: std::io::Error) {
    panic!("{}", error);
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for byte in io::stdin().bytes() {
        match byte {
            Ok(byte) => {
                let char = byte as char;

                if char.is_control() {
                    println!("{:#b} \r", byte)
                } else {
                    println!("{:#b} ({})\r", byte, char)
                }

                if byte == to_ctrl_byte('z') {
                    break;
                }
            },
            Err(error) => die(error),
        }
    }
}
