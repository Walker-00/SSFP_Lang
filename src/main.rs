use lazy_static::lazy_static;
use std::{
    io::{stdin, Read},
    sync::Mutex,
};

#[derive(Debug)]
#[repr(i8)]
enum Token {
    Tokother(i8),
    Tokeof = -1,
    Tokfun = -2,
    Tokextern = -3,
    Tokidentifier = -4,
    Toknumber = -5,
    Tokcommand = -6,
}

lazy_static! {
    static ref IDENTIFIERSTR: Mutex<String> = Mutex::new(String::default());
    static ref NUMVAL: Mutex<f64> = Mutex::new(f64::default());
}

fn gettok() -> Token {
    let mut identifierstr = IDENTIFIERSTR.lock().unwrap();
    let mut numval = NUMVAL.lock().unwrap();
    let mut last_char = ' ';
    let mut input = [0; 1];
    while last_char.is_ascii_whitespace() {
        stdin().read_exact(&mut input).unwrap();
        last_char = input[0] as char;
    }

    if last_char.is_ascii_alphabetic() {
        *identifierstr = last_char.to_string();

        loop {
            if stdin().read_exact(&mut input).is_ok() {
                last_char = input[0] as char;
                if last_char.is_ascii_alphanumeric() {
                    identifierstr.push(last_char);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        if identifierstr.as_str() == "fun" {
            Token::Tokfun
        } else if identifierstr.as_str() == "extern" {
            Token::Tokextern
        } else {
            Token::Tokidentifier
        }
    } else if last_char.is_ascii_digit() || last_char == '.' {
        let mut num_str: String = String::default();
        while last_char.is_ascii_digit() || last_char == '.' {
            num_str.push(last_char);
            stdin().read_exact(&mut input).unwrap();
            last_char = input[0] as char;
        }
        *numval = num_str.parse().unwrap();
        Token::Toknumber
    } else if last_char == '#' {
        while last_char as u8 != 0 && last_char != '\n' && last_char != '\r' {
            stdin().read_exact(&mut input).unwrap();
            last_char = input[0] as char;
        }
        if last_char == '\n' {
            Token::Tokcommand
        } else if last_char as u8 != 0 {
            gettok()
        } else {
            Token::Tokeof
        }
    } else if last_char as u8 == 0 {
        Token::Tokeof
    } else {
        let this_char = last_char;
        stdin().read_exact(&mut input).unwrap();
        return Token::Tokother(this_char as i8);
    }
}

fn main() {
    loop {
        let tok: Token = gettok();
        println!("got token: {tok:?}");
    }
}
