use std::fs::read_to_string;
use std::io;

pub const SYNOPSIS: &str = "fcat [FILE]";

pub fn run(v: &[String]) -> io::Result<()> {
    for arg in v {
        match read_to_string(arg) {
            Err(e) => return Err(e),
            Ok(s) => print!("{}", s),
        }
    }
    Ok(())
}
