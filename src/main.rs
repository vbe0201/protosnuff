extern crate ansi_term;
extern crate getopts;

use ansi_term::Color::Red;
use std::env;

mod arguments;
mod io;
mod parser;

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
}
