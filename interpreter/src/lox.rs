use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;
use std::process;

use anyhow::Context;

use crate::error::LoxError;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::{Token, TokenType};

pub struct Lox {
    interpreter: Interpreter,
    had_error: bool,
    had_runtime_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self {
            had_error: false,
            had_runtime_error: false,
            interpreter: Interpreter::new(),
        }
    }

    pub fn run_file(&mut self, source_file: &Path) -> anyhow::Result<()> {
        let contents = fs::read_to_string(source_file)
            .with_context(|| format!("could not read file `{}`", source_file.to_string_lossy()))?;

        if let Err(err) = self.run(&contents) {
            eprintln!("Error running file: {}", err);

            process::exit(exitcode::DATAERR);
        };

        // highly inconsistent with had_error handling
        if self.had_runtime_error {
            process::exit(exitcode::SOFTWARE);
        }

        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let mut line = String::new();

            eprint!("> ");

            if io::stdin()
                .read_line(&mut line)
                .with_context(|| "unable to read stdin".to_owned())?
                == 0
            {
                break;
            }

            let _ = self.run(&line);
            self.had_error = false;
        }

        eprintln!();

        Ok(())
    }

    fn run(&mut self, source: &str) -> Result<(), LoxError> {
        eprintln!("running source:\n{}", source);

        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens(self);

        if self.had_error {
            return Err(LoxError::Error(
                "encountered error(s) during scanning".to_owned(),
            ));
        }

        let mut parser = Parser::new(tokens);
        let statements = parser.parse(self);

        if self.had_error {
            return Err(LoxError::Error(
                "encountered error(s) during parsing".to_owned(),
            ));
        }

        if let Err(err) = self.interpreter.interpret(&statements) {
            match err {
                LoxError::Runtime { message, token } => {
                    self.runtime_error(&message, &token);
                }
                e => return Err(e),
            }
        };

        Ok(())
    }

    pub fn error(&mut self, line: i32, message: &str) {
        self.report(line, "", message);
    }

    pub fn parser_error(&mut self, token: &Token, message: &str) {
        if token.token_type == TokenType::Eof {
            self.report(token.line, "at end", message);
        } else {
            self.report(token.line, &format!("at '{}'", token.lexeme), message);
        }
    }

    pub fn runtime_error(&mut self, message: &str, token: &Token) {
        eprintln!("{}\n[line {}]", message, token.line);

        self.had_runtime_error = true;
    }

    fn report(&mut self, line: i32, place: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, place, message);

        self.had_error = true;
    }
}
