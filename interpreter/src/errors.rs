use std::fmt;

use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

impl fmt::Display for ParseError {
    // NOTE: this code is never used
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.token.token_type == TokenType::Eof {
            write!(f, " at end")
        } else {
            write!(f, " at '{}'", self.token.lexeme)
        }
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl fmt::Display for RuntimeError {
    // NOTE: this code is never used
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RuntimeError at {}", self.token.lexeme)
    }
}
