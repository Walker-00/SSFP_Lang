use std::io::{stdin, Read};

enum Token {
    TOKEOF = -1,
    TOKDEF = -2,
    TOKEXTERN = -3,
    TOKIDENTIFIER = -4,
    TOKNUMBER = -5,
}

static mut IDENTIFIERSTR: String = String::new();
static mut NUMVAL: f64 = 0.;

fn gettok() -> i32 {
    let mut last_char: u8 = b' ';
    while last_char.is_ascii_whitespace() {
        let mut input = [0; 1];
        stdin().read_exact(&mut input).unwrap();
    }
    2
}

fn main() {
    println!("Hello, world!");
}
