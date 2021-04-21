#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character tokens
    // (
    LeftParen,
    // )
    RightParen,
    // {
    LeftBrace,
    // }
    RightBrace,
    // ,
    Comma,
    // .
    Dot,
    // -
    Minus,
    // +
    Plus,
    // ;
    Semicolon,
    // /
    Slash,
    // *
    Star,

    // One or two character tokens
    // !
    Bang,
    // !=
    BangEqual,
    // =
    Equal,
    // ==
    EqualEqual,
    // >
    Greater,
    // >=
    GreaterEqual,
    // <
    Less,
    // <=
    LessEqual,

    // Literals.
    Identifier,
    String { literal: String },
    Number { literal: f64 },

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,

    Unknown,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: i32) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }

    // TODO: to_string
}
