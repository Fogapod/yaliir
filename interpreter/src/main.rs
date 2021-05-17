// mod astprinter;
mod callable;
mod environment;
mod error;
mod expression;
mod function;
mod interpreter;
mod lox;
mod object;
mod parser;
mod scanner;
mod statement;
mod token;

use std::cmp::Ordering;
use std::env;
use std::error::Error;
use std::path::Path;
use std::process;

use lox::Lox;

fn main() -> Result<(), Box<dyn Error>> {
    let mut lox = Lox::new();

    let argv = env::args().collect::<Vec<String>>();

    match argv.len().cmp(&2) {
        Ordering::Equal => {
            let source_file = Path::new(&argv[1]);
            lox.run_file(&source_file)?;
        }
        Ordering::Less => lox.run_prompt()?,
        Ordering::Greater => {
            eprintln!("Usage: yaliir [script]");

            process::exit(exitcode::USAGE);
        }
    }

    Ok(())
}
