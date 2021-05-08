use anyhow::{Context, Result};
use std::fs;
use std::io;
use std::path::Path;
use std::process;

use crate::expression::AstPrinter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn run_file(&mut self, source_file: &Path) -> Result<()> {
        let contents = fs::read_to_string(source_file)
            .with_context(|| format!("could not read file `{}`", source_file.to_string_lossy()))?;

        if let Err(err) = self.run(&contents) {
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
                .with_context(|| "unable to read stdin".to_string())?
                == 0
            {
                break;
            }

            let _ = self.run(&line);
        }

        eprintln!();

        Ok(())
    }

    fn run(&mut self, source: &str) -> Result<()> {
        eprintln!("running source:\n{}", source);

        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens(self);

        if self.had_error {
            anyhow::bail!("encountered errors during scanning");
        }

        let mut parser = Parser::new(tokens);
        let expression = parser.parse(self);

        if self.had_error {
            anyhow::bail!("encountered errors during parser");
        }

        match expression {
            Some(expression) => println!("{}", AstPrinter {}.print(expression)),
            None => println!("nothing parsed"),
        }

        Ok(())
    }

    pub fn error(&mut self, line: i32, message: &str) {
        self.report(line, "", message);
    }

    pub fn parser_error(&mut self, token: &Token, message: &str) {
        if token.token_type == TokenType::Eof {
            self.report(token.line, " at end", message);
        } else {
            self.report(token.line, &format!(" at '{}'", token.lexeme), message);
        }
    }

    fn report(&mut self, line: i32, place: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, place, message);

        self.had_error = true;
    }
}
