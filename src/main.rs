mod cat;
mod echo;

use std::env;
use std::io::{Error, ErrorKind};

const NAME: &str = "rustil";

mod status {

    /// Exit status code for when execution OK.
    pub const SUCCESS: i32 = 0;

    /// Exit status code for when there was at least one error during execution.
    pub const RUNTIME_ERROR: i32 = 1;

    /// Exit status code for when passed no argument on starting.
    pub const INVALID_ARGUMENT: i32 = 2;
}

fn usage() {
    println!(
        "{} is a tool for embedding new features by Rust source code.\n",
        NAME
    );
    println!("The execution commands are:");
    println!("\t- {}", echo::SYNOPSIS);
    println!("\t- {}", cat::SYNOPSIS);
}

fn run(args: &[String]) -> std::io::Result<i32> {
    let cmd = args[0].as_str();
    match cmd {
        "echo" => match echo::run(&args[1..]) {
            Ok(()) => return Ok(status::SUCCESS),
            Err(e) => return Err(e),
        },
        "cat" => match cat::run(&args[1..]) {
            Ok(()) => return Ok(status::SUCCESS),
            Err(e) => return Err(e),
        },
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "invalid parameter was given",
            ));
        }
    };
}

fn main() {
    use std::process::exit;

    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        usage();
        exit(status::INVALID_ARGUMENT);
    }

    let result = run(&args);
    match result {
        Ok(code) => {
            exit(code);
        }
        Err(e) => {
            eprintln!("{}: {}", NAME, e);
            exit(status::RUNTIME_ERROR);
        }
    }
}
