use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub const SYNOPSIS: &str = "grep [PATTERN] [FILE]...";

pub fn run(args: &[String]) -> io::Result<()> {
    let pattern = &args[0];

    for arg in args[1..].iter() {
        let file = match File::open(arg) {
            Err(e) => return Err(e),
            Ok(file) => file,
        };

        let reader = BufReader::new(file);
        for line in reader.lines() {
            let l = line?;
            if l.contains(pattern) {
                println!("{}:{}", arg, l);
            }
        }
    }
    Ok(())
}
