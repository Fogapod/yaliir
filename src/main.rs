mod error;
mod lox;
mod scanner;
mod token;

use anyhow::Result;
use exitcode;
use human_panic::setup_panic;
use std::env;
use std::process;

use lox::Lox;

fn main() -> Result<()> {
    setup_panic!();

    let mut lox = Lox::new();

    let argv = env::args().collect::<Vec<String>>();

    let result = if argv.len() > 2 {
        eprintln!("Usage: yaliir [script]");

        process::exit(exitcode::USAGE);
    } else if argv.len() == 2 {
        lox.run_file(&argv[1])
    } else {
        lox.run_prompt()
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    };

    Ok(())
}
