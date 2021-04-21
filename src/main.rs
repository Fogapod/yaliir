mod error;
mod lox;
mod scanner;
mod token;

use human_panic::setup_panic;
use std::cmp::Ordering;
use std::env;
use std::process;

use lox::Lox;

fn main() {
    setup_panic!();

    let mut lox = Lox::new();

    let argv = env::args().collect::<Vec<String>>();

    let result = match argv.len().cmp(&2) {
        Ordering::Equal => lox.run_file(&argv[1]),
        Ordering::Less => lox.run_prompt(),
        Ordering::Greater => {
            eprintln!("Usage: yaliir [script]");

            process::exit(exitcode::USAGE);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    };
}
