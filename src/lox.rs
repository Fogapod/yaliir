use crate::scanner::Scanner;
use anyhow::{Context, Result};
use exitcode;
use std::fs;
use std::io;
use std::process;

pub struct Lox {
    //had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { /*had_error: false*/ }
    }

    pub fn run_file(&self, filename: &String) -> Result<()> {
        let contents = fs::read_to_string(filename)
            .with_context(|| format!("could not read file `{}`", filename))?;

        if let Err(err) = self.run(contents) {
            eprintln!("Error running file: {}", err);

            process::exit(exitcode::DATAERR);
        };

        Ok(())
    }
    pub fn run_prompt(&mut self) -> Result<()> {
        loop {
            let mut line = String::new();

            eprint!("> ");

            if io::stdin()
                .read_line(&mut line)
                .with_context(|| format!("unable to read stdin"))?
                == 0
            {
                break;
            }

            // ignore result with possible error
            let _ = self.run(line);
        }

        eprintln!();

        Ok(())
    }

    fn run(&self, source: String) -> Result<()> {
        eprintln!("running source:\n{}", source);

        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;

        for token in tokens {
            eprintln!("{:?}", token);
        }

        Ok(())
    }
}
