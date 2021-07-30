use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub const SYNOPSIS: &str = "cat [-n|--number] [FILE]";

pub fn run(v: &[String]) -> io::Result<()> {
    let mut with_number = false;
    for arg in v {
        if !is_option(arg) {
            continue;
        }
        if arg == "-n" || arg == "--number" {
            with_number = true;
        }
    }

    for arg in v {
        if is_option(arg) {
            continue;
        }

        let file = Path::new(arg);
        if !file.exists() {
            eprintln!("rat: {}: No such file or directory", arg);
            continue;
        }

        if file.is_dir() {
            eprintln!("rat: {}: Is a directory", arg);
            continue;
        }

        if file.is_file() {
            if with_number {
                output_with_number(arg)?;
            } else {
                output(arg)?;
            }
        }
    }
    Ok(())
}

fn is_option(name: &str) -> bool {
    let bytes = name.as_bytes();
    bytes[0] == 45
}

fn output(filepath: &str) -> io::Result<()> {
    let f = File::open(filepath)?;
    let reader = BufReader::new(f);
    let mut stdout = io::stdout();
    for line in reader.lines() {
        writeln!(stdout, "{}", line?)?;
    }
    Ok(())
}

fn output_with_number(filepath: &str) -> io::Result<()> {
    let f = File::open(filepath)?;
    let reader = BufReader::new(f);
    let mut stdout = io::stdout();
    let mut num = 1;
    for line in reader.lines() {
        writeln!(stdout, "{0: >6}  {1}", num, line?)?;
        num += 1;
    }
    Ok(())
}
