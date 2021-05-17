use thiserror::Error;

use crate::object::Object;
use crate::token::Token;

#[derive(Debug, Error)]
pub enum LoxError {
    #[error("Error")]
    Error(String),

    #[error("RuntimeError")]
    Runtime { token: Token, message: String },

    #[error("Function return")]
    Return(Object),

    #[error("ParserError")]
    Parser,
}
