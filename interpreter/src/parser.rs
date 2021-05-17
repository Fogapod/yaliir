use crate::expression::Expr;
use crate::lox::Lox;
use crate::object::Object;
use crate::statement::Stmt;
use crate::token::{Token, TokenType};

use crate::error::LoxError;

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
            if let Some(declaration) = self.declaration(lox) {
                statements.push(declaration);
            }
        }

        statements
    }

    fn expression(&mut self, lox: &mut Lox) -> Result<Expr, LoxError> {
        self.assignment(lox)
    }

    fn declaration(&mut self, lox: &mut Lox) -> Option<Stmt> {
        let stmt = if self.match_(&[TokenType::Fun]) {
            self.function("function", lox)
        } else if self.match_(&[TokenType::Var]) {
            self.var_declaration(lox)
        } else {
            self.statement(lox)
        };

        match stmt {
            Err(_) => {
                self.synchronize();

                None
            }
            Ok(stmt) => Some(stmt),
        }
    }

    fn statement(&mut self, lox: &mut Lox) -> Result<Stmt, LoxError> {
        if self.match_(&[TokenType::For]) {
            self.for_statement(lox)
        } else if self.match_(&[TokenType::If]) {
            self.if_statement(lox)
        } else if self.match_(&[TokenType::Print]) {
            self.print_statement(lox)
        } else if self.match_(&[TokenType::Return]) {
            self.return_statement(lox)
        } else if self.match_(&[TokenType::While]) {
            self.while_statement(lox)
        } else if self.match_(&[TokenType::LeftBrace]) {
            Ok(Stmt::Block {
                statements: self.block(lox)?,
            })
        } else {
            self.expression_statement(lox)
        }
    }

    fn for_statement(&mut self, lox: &mut Lox) -> Result<Stmt, LoxError> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'for'.", lox)?;

        let initializer = if self.match_(&[TokenType::Semicolon]) {
            None
        } else if self.match_(&[TokenType::Var]) {
            Some(self.var_declaration(lox)?)
        } else {
            Some(self.expression_statement(lox)?)
        };

        let condition = if self.check(&TokenType::Semicolon) {
            None
        } else {
            Some(self.expression(lox)?)
        };

        self.consume(
            &TokenType::Semicolon,
            "Expect ';' after loop condition.",
            lox,
        )?;

        let increment = if self.check(&TokenType::RightParen) {
            None
        } else {
            Some(self.expression(lox)?)
        };

        self.consume(&TokenType::RightParen, "Expect ')' after for clauses.", lox)?;

        let mut body = self.statement(lox)?;

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

    fn if_statement(&mut self, lox: &mut Lox) -> Result<Stmt, LoxError> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'if'.", lox)?;

        let condition = self.expression(lox)?;

        self.consume(&TokenType::RightParen, "Expect ')' after 'if'.", lox)?;

        let then_branch = self.statement(lox)?;
        let mut else_branch = None;

        if self.match_(&[TokenType::Else]) {
            else_branch = Some(Box::new(self.statement(lox)?));
        }

        Ok(Stmt::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    fn print_statement(&mut self, lox: &mut Lox) -> Result<Stmt, LoxError> {
        let value = self.expression(lox)?;

        self.consume(&TokenType::Semicolon, "Expect ';' after value.", lox)?;

        Ok(Stmt::Print { value })
    }

    fn return_statement(&mut self, lox: &mut Lox) -> Result<Stmt, LoxError> {
        let keyword = self.previous();

        let value = if self.check(&TokenType::Semicolon) {
            None
        } else {
            Some(self.expression(lox)?)
        };

        self.consume(&TokenType::Semicolon, "Expect ';' after return value.", lox)?;

        Ok(Stmt::Return { keyword, value })
    }

    fn var_declaration(&mut self, lox: &mut Lox) -> Result<Stmt, LoxError> {
        let name = self.consume(&TokenType::Identifier, "Expect variable name.", lox)?;

        let initializer = if self.match_(&[TokenType::Equal]) {
            Some(self.expression(lox)?)
        } else {
            None
        };

        self.consume(
            &TokenType::Semicolon,
            "Expect ';' after variable declaration.",
            lox,
        )?;

        Ok(Stmt::Var { name, initializer })
    }

    fn while_statement(&mut self, lox: &mut Lox) -> Result<Stmt, LoxError> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'while'.", lox)?;

        let condition = self.expression(lox)?;

        self.consume(&TokenType::RightParen, "Expect ')' after condition.", lox)?;

        let body = self.statement(lox)?;

        Ok(Stmt::While {
            condition,
            body: Box::new(body),
        })
    }

    fn expression_statement(&mut self, lox: &mut Lox) -> Result<Stmt, LoxError> {
        let value = self.expression(lox)?;

        self.consume(&TokenType::Semicolon, "Expect ';' after value.", lox)?;

        Ok(Stmt::Expression { value })
    }

    fn function(&mut self, kind: &str, lox: &mut Lox) -> Result<Stmt, LoxError> {
        let name = self.consume(
            &TokenType::Identifier,
            &format!("Expect {} name.", kind),
            lox,
        )?;

        self.consume(
            &TokenType::LeftParen,
            &format!("Expect '(' after {} name.", kind),
            lox,
        )?;

        let mut params = vec![];

        if !self.check(&TokenType::RightParen) {
            loop {
                if params.len() > 255 {
                    self.error(&self.peek(), "Can't have more than 255 parameters.", lox);
                }

                params.push(self.consume(&TokenType::Identifier, "Expect parameter name.", lox)?);

                if !self.match_(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(&TokenType::RightParen, "Expect ')' after parameters.", lox)?;
        self.consume(
            &TokenType::LeftBrace,
            &format!("Expect '{{' after {} body.", kind),
            lox,
        )?;

        let body = self.block(lox)?;

        Ok(Stmt::Function { name, params, body })
    }

    fn block(&mut self, lox: &mut Lox) -> Result<Vec<Stmt>, LoxError> {
        let mut statements = vec![];

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if let Some(declaration) = self.declaration(lox) {
                statements.push(declaration);
            }
        }

        self.consume(&TokenType::RightBrace, "Expect '}' after block.", lox)?;

        Ok(statements)
    }

    fn assignment(&mut self, lox: &mut Lox) -> Result<Expr, LoxError> {
        let expr = self.or(lox)?;

        if self.match_(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment(lox)?;

            match expr {
                Expr::Variable { name } => {
                    return Ok(Expr::Assign {
                        name,
                        value: Box::new(value),
                    });
                }
                _ => self.error(&equals, "Invalid assignment target.", lox),
            };
        }

        Ok(expr)
    }

    fn or(&mut self, lox: &mut Lox) -> Result<Expr, LoxError> {
        let mut expr = self.and(lox)?;

        while self.match_(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.and(lox)?;

            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&mut self, lox: &mut Lox) -> Result<Expr, LoxError> {
        let mut expr = self.equality(lox)?;

        while self.match_(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.equality(lox)?;

            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self, lox: &mut Lox) -> Result<Expr, LoxError> {
        let mut expr = self.comparison(lox)?;

        while self.match_(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison(lox)?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self, lox: &mut Lox) -> Result<Expr, LoxError> {
        let mut expr = self.term(lox)?;

        while self.match_(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term(lox)?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self, lox: &mut Lox) -> Result<Expr, LoxError> {
        let mut expr = self.factor(lox)?;

        while self.match_(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor(lox)?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self, lox: &mut Lox) -> Result<Expr, LoxError> {
        let mut expr = self.unary(lox)?;

        while self.match_(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary(lox)?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self, lox: &mut Lox) -> Result<Expr, LoxError> {
        if self.match_(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary(lox)?;

            Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            })
        } else {
            self.call(lox)
        }
    }

    fn finish_call(&mut self, callee: Expr, lox: &mut Lox) -> Result<Expr, LoxError> {
        let mut arguments = vec![];

        if !self.check(&TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    self.error(&self.peek(), "Can't have more than 255 arguments.", lox);
                }

                arguments.push(self.expression(lox)?);

                if !self.match_(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(&TokenType::RightParen, "Expect ')' after arguments.", lox)?;

        Ok(Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }

    fn call(&mut self, lox: &mut Lox) -> Result<Expr, LoxError> {
        let mut expr = self.primary(lox)?;

        loop {
            if self.match_(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr, lox)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn primary(&mut self, lox: &mut Lox) -> Result<Expr, LoxError> {
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
                let expr = self.expression(lox)?;

                self.consume(&TokenType::RightParen, "Expect ')' after expression.", lox)?;

                Expr::Grouping {
                    expression: Box::new(expr),
                }
            }
            _ => {
                return Err(self.error(&current, "Expect expression.", lox));
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

    fn consume(
        &mut self,
        token_type: &TokenType,
        message: &str,
        lox: &mut Lox,
    ) -> Result<Token, LoxError> {
        if !self.check(token_type) {
            return Err(self.error(&self.peek(), message, lox));
        }

        Ok(self.advance())
    }

    fn error(&mut self, token: &Token, message: &str, lox: &mut Lox) -> LoxError {
        lox.parser_error(token, message);

        LoxError::Parser
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
