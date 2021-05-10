use crate::expression::Expr;
use crate::lox::Lox;
use crate::object::Object;
use crate::statement::Stmt;
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

    pub fn parse(&mut self, lox: &mut Lox) -> Vec<Stmt> {
        let mut statements = vec![];

        while !self.is_at_end() {
            match self.declaration() {
                Err(err) => {
                    if let Some(parser_error) = err.downcast_ref::<ParseError>() {
                        lox.parser_error(&parser_error.token, &parser_error.message);
                    } else {
                        panic!("unknown parsing error: {:?}", err);
                    }
                }
                Ok(statement) => statements.push(statement),
            }
        }

        statements
    }

    fn expression(&mut self) -> anyhow::Result<Expr> {
        self.assignment()
    }

    fn declaration(&mut self) -> anyhow::Result<Stmt> {
        let stmt = if self.match_(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        match stmt {
            Err(e) => {
                self.synchronize();
                anyhow::bail!(e);
            }
            stmt => stmt,
        }
    }

    fn statement(&mut self) -> anyhow::Result<Stmt> {
        if self.match_(&[TokenType::For]) {
            self.for_statement()
        } else if self.match_(&[TokenType::If]) {
            self.if_statement()
        } else if self.match_(&[TokenType::Print]) {
            self.print_statement()
        } else if self.match_(&[TokenType::While]) {
            self.while_statement()
        } else if self.match_(&[TokenType::LeftBrace]) {
            Ok(Stmt::Block {
                statements: self.block()?,
            })
        } else {
            self.expression_statement()
        }
    }

    fn for_statement(&mut self) -> anyhow::Result<Stmt> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer = if self.match_(&[TokenType::Semicolon]) {
            None
        } else if self.match_(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if self.check(&TokenType::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(&TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if self.check(&TokenType::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(&TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Stmt::Block {
                statements: vec![body, Stmt::Expression { value: increment }],
            };
        }

        body = Stmt::While {
            condition: match condition {
                Some(condition) => condition,
                None => Expr::Literal {
                    object: Object::Boolean(true),
                },
            },
            body: Box::new(body),
        };

        if let Some(initializer) = initializer {
            body = Stmt::Block {
                statements: vec![initializer, body],
            };
        }

        Ok(body)
    }

    fn if_statement(&mut self) -> anyhow::Result<Stmt> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'if'.")?;

        let condition = self.expression()?;

        self.consume(&TokenType::RightParen, "Expect ')' after 'if'.")?;

        let then_branch = self.statement()?;
        let mut else_branch = None;

        if self.match_(&[TokenType::Else]) {
            else_branch = Some(Box::new(self.statement()?));
        }

        Ok(Stmt::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    fn print_statement(&mut self) -> anyhow::Result<Stmt> {
        let value = self.expression()?;

        self.consume(&TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(Stmt::Print { value })
    }

    fn var_declaration(&mut self) -> anyhow::Result<Stmt> {
        let name = self.consume(&TokenType::Identifier, "Expect variable name.")?;

        let initializer = if self.match_(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            &TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::Var { name, initializer })
    }

    fn while_statement(&mut self) -> anyhow::Result<Stmt> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'while'.")?;

        let condition = self.expression()?;

        self.consume(&TokenType::RightParen, "Expect ')' after condition.")?;

        let body = self.statement()?;

        Ok(Stmt::While {
            condition,
            body: Box::new(body),
        })
    }

    fn expression_statement(&mut self) -> anyhow::Result<Stmt> {
        let value = self.expression()?;

        self.consume(&TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(Stmt::Expression { value })
    }

    fn block(&mut self) -> anyhow::Result<Vec<Stmt>> {
        let mut statements = vec![];

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(&TokenType::RightBrace, "Expect '}' after block.")?;

        Ok(statements)
    }

    fn assignment(&mut self) -> anyhow::Result<Expr> {
        let expr = self.or()?;

        if self.match_(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            match expr {
                Expr::Variable { name } => {
                    return Ok(Expr::Assign {
                        name,
                        value: Box::new(value),
                    })
                }
                _ => anyhow::bail!(ParseError {
                    token: equals,
                    message: "Invalid assignment target.".to_string()
                }),
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> anyhow::Result<Expr> {
        let mut expr = self.and()?;

        while self.match_(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;

            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> anyhow::Result<Expr> {
        let mut expr = self.equality()?;

        while self.match_(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;

            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
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
        let current = self.peek();

        let expr = match current.token_type {
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
            TokenType::Identifier => Expr::Variable { name: current },
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
                    token: current,
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
                _ => self.advance(),
            };
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().token_type == token_type
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
