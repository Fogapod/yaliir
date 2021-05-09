mod errors;
mod expression;
mod interpreter;
mod lox;
mod object;
mod parser;
mod scanner;
mod token;

use std::cmp::Ordering;
use std::env;
use std::path::Path;
use std::process;

use lox::Lox;

fn main() {
    let mut lox = Lox::new();

    let argv = env::args().collect::<Vec<String>>();

    let result = match argv.len().cmp(&2) {
        Ordering::Equal => {
            let source_file = Path::new(&argv[1]);
            lox.run_file(&source_file)
        }
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
