use crate::expression::Expr;
use crate::lox::Lox;
use crate::object::Object;
use crate::token::{Token, TokenType};

use crate::errors::ParseError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Self {
            tokens: tokens.to_vec(),
            current: 0,
        }
    }

    pub fn parse(&mut self, lox: &mut Lox) -> Option<Expr> {
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(err) => {
                if let Some(parser_error) = err.downcast_ref::<ParseError>() {
                    lox.parser_error(&parser_error.token, &parser_error.message);
                } else {
                    panic!("unknown parsing error: {:?}", err);
                }

                None
            }
        }
    }

    fn expression(&mut self) -> anyhow::Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> anyhow::Result<Expr> {
        let mut expr = self.comparison()?;

        while self.match_(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> anyhow::Result<Expr> {
        let mut expr = self.term()?;

        while self.match_(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> anyhow::Result<Expr> {
        let mut expr = self.factor()?;

        while self.match_(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> anyhow::Result<Expr> {
        let mut expr = self.unary()?;

        while self.match_(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> anyhow::Result<Expr> {
        if self.match_(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;

            Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> anyhow::Result<Expr> {
        // much cleaner than using self.match_
        // the downside is having to call self.advance manually
        let expr = match self.peek().token_type {
            TokenType::False => Expr::Literal {
                object: Object::Boolean(false),
            },
            TokenType::True => Expr::Literal {
                object: Object::Boolean(true),
            },
            TokenType::Nil => Expr::Literal {
                object: Object::Null,
            },
            TokenType::Number { literal } => Expr::Literal {
                object: Object::Number(literal),
            },
            TokenType::String { literal } => Expr::Literal {
                object: Object::String(literal),
            },
            TokenType::LeftParen => {
                // NOTE: this causes infinite recursion!!!
                // probably fixed in next chapters
                let expr = self.expression()?;

                self.consume(&TokenType::RightParen, "Expect ')' after expression.")?;

                Expr::Grouping {
                    expression: Box::new(expr),
                }
            }
            _ => {
                anyhow::bail!(ParseError {
                    token: self.peek(),
                    message: "Expect expression.".to_string()
                });
            }
        };

        self.advance();

        Ok(expr)
    }

    fn match_(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> anyhow::Result<Token> {
        if !self.check(token_type) {
            anyhow::bail!(ParseError {
                token: self.peek(),
                message: message.to_string()
            });
        }

        Ok(self.advance())
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == *token_type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens
            .get(self.current)
            .expect("exhausted token array")
            .clone()
    }

    fn previous(&self) -> Token {
        self.tokens
            .get(self.current - 1)
            .expect("exhausted token array")
            .clone()
    }
}
