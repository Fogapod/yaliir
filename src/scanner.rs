use crate::error::error;
use crate::token::{Token, TokenType};
use anyhow::{anyhow, Result};
use phf::phf_map;

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false"=>TokenType::False,
    "fun"=>TokenType::Fun,
    "for"=>TokenType::For,
    "if"=>TokenType::If,
    "mil"=>TokenType::Nil,
    "or"=>TokenType::Or,
    "print"=>TokenType::Print,
    "return"=>TokenType::Return,
    "super"=>TokenType::Super,
    "this"=>TokenType::This,
    "true"=>TokenType::True,
    "var"=>TokenType::Var,
    "while"=>TokenType::While,
};

#[derive(Debug)]
pub struct Scanner {
    source_str: String,
    source: Vec<char>,
    tokens: Vec<Token>,

    // TODO: Move to struct
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source_str: source.clone(),
            source: source.chars().collect(),
            tokens: vec![],

            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let ch = self.source[self.current];

        self.current += 1;

        ch
    }

    fn add_token(&mut self, token: TokenType) {
        let text = self
            .source_str
            .get(self.start as usize..self.current as usize)
            .unwrap()
            .to_string();

        self.tokens.push(Token::new(token, text, self.line));
    }

    fn match_(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        };

        if self.source[self.current] != expected {
            return false;
        };

        self.current += 1;

        true
    }

    fn peek(&self, offset: usize) -> char {
        let pos = self.current + offset;
        if pos >= self.source.len() {
            '\0'
        } else {
            self.source[pos]
        }
    }

    fn string(&mut self) -> Result<()> {
        while self.peek(0) != '"' && !self.is_at_end() {
            if self.peek(0) == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(anyhow!("Unterminated string."));
        }

        self.advance();

        Ok(self.add_token(TokenType::String {
            literal: self
                .source_str
                .get(self.start + 1..self.current - 1)
                .unwrap()
                .to_string(),
        }))
    }

    fn is_digit(c: char) -> bool {
        c.is_digit(10)
    }

    fn number(&mut self) {
        while Self::is_digit(self.peek(0)) {
            self.advance();
        }

        if self.peek(0) == '.' && Self::is_digit(self.peek(1)) {
            self.advance();
        }

        self.add_token(TokenType::Number {
            literal: self
                .source_str
                .get(self.start..self.current)
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
        })
    }

    fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alphanumeric(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    fn identifier(&mut self) {
        while Self::is_alphanumeric(self.peek(0)) {
            self.advance();
        }

        let text = self
            .source_str
            .get(self.start..self.current)
            .unwrap()
            .to_string();

        self.add_token({
            if let Some(token) = KEYWORDS.get(&text[..]) {
                (*token).clone()
            } else {
                TokenType::Identifier
            }
        })
    }

    fn scan_token(&mut self) -> Result<()> {
        let c = self.advance();

        let maybe_token = match c {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            ';' => Some(TokenType::Semicolon),
            '*' => Some(TokenType::Star),
            '!' => {
                if self.match_('=') {
                    Some(TokenType::BangEqual)
                } else {
                    Some(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_('=') {
                    Some(TokenType::EqualEqual)
                } else {
                    Some(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_('=') {
                    Some(TokenType::LessEqual)
                } else {
                    Some(TokenType::Less)
                }
            }
            '>' => {
                if self.match_('=') {
                    Some(TokenType::GreaterEqual)
                } else {
                    Some(TokenType::Greater)
                }
            }
            '/' => {
                if self.match_('/') {
                    while self.peek(0) != '\n' && !self.is_at_end() {
                        self.advance();
                    }

                    None
                } else {
                    Some(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => None,
            '"' => {
                self.string()?;
                None
            }
            '\n' => {
                self.line += 1;
                None
            }
            _ => {
                if Self::is_digit(c) {
                    self.number();
                    None
                } else if Self::is_alpha(c) {
                    self.identifier();
                    None
                } else {
                    Some(TokenType::Unknown)
                }
            }
        };

        if let Some(token) = maybe_token {
            if token == TokenType::Unknown {
                Err(anyhow!("Unexpected character: {}", c))
            } else {
                Ok(self.add_token(token))
            }
        } else {
            Ok(())
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>> {
        let mut num_errors = 0;

        while !self.is_at_end() {
            self.start = self.current;

            if let Err(err) = self.scan_token() {
                error(self.line, err.to_string());
                num_errors += 1;
            }
        }
        self.tokens
            .push(Token::new(TokenType::Eof, String::from(""), self.line));

        if num_errors > 0 {
            Err(anyhow!(
                "Encountered {} error(s) during token scanning",
                num_errors
            ))
        } else {
            Ok(&self.tokens)
        }
    }
}
