extern crate ansi_term;
extern crate getopts;

use ansi_term::Color::Red;
use std::env;

mod arguments;

fn main() {
    // Get and parse arguments.
    let args: Vec<String> = env::args().collect();
    let arguments = match arguments::Arguments::parse(&args) {
        Ok(a) => { a }
        Err(f) => {
            // Any empty error here is considered an early bail.
            if f.is_empty() {
                return;
            }

            // Otherwise, print the error.
            eprintln!("{}", Red.paint(f));
            return;
        }
    };

    // Check if provided input file exists and print contents.
    if !arguments.input.exists() {
        eprintln!("{} {}",
                  Red.paint(arguments.input.to_str().unwrap()),
                  Red.paint("does not exist!"));
        return;
    }
    println!("Input: {:?}", std::fs::read_to_string(&arguments.input).unwrap());
}
