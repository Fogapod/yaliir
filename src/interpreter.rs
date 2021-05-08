use std::convert::{TryFrom, TryInto};

use crate::errors::RuntimeError;
use crate::expression::{Expr, Visitor};
use crate::object::Object;
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(&mut self, expression: &Expr) -> anyhow::Result<()> {
        println!("{}", self.evaluate(expression)?);

        Ok(())
    }

    fn evaluate(&self, expr: &Expr) -> anyhow::Result<Object> {
        expr.accept(self)
    }
}

impl Visitor<anyhow::Result<Object>> for Interpreter {
    fn visit_assign(&self, name: &Token, value: &Expr) -> anyhow::Result<Object> {
        todo!();
    }
    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> anyhow::Result<Object> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        Ok(match operator.token_type {
            TokenType::Minus => Object::Number(f64::try_from(left)? - f64::try_from(right)?),
            TokenType::Slash => Object::Number(f64::try_from(left)? / f64::try_from(right)?),
            TokenType::Star => Object::Number(f64::try_from(left)? * f64::try_from(right)?),
            TokenType::Plus => match (&left, &right) {
                (Object::Number(number1), Object::Number(number2)) => {
                    Object::Number(number1 + number2)
                }
                (Object::String(string1), Object::String(string2)) => {
                    Object::String(string1.to_owned() + string2)
                }
                _ => anyhow::bail!(RuntimeError {
                    token: operator.clone(),
                    message: "Operands must be two numbers or two strings.".to_string()
                }),
            },
            TokenType::Greater => Object::Boolean(f64::try_from(left)? > right.try_into()?),
            TokenType::GreaterEqual => Object::Boolean(f64::try_from(left)? >= right.try_into()?),
            TokenType::Less => Object::Boolean(f64::try_from(left)? < right.try_into()?),
            TokenType::LessEqual => Object::Boolean(f64::try_from(left)? <= right.try_into()?),
            TokenType::BangEqual => Object::Boolean(left != right),
            TokenType::EqualEqual => Object::Boolean(left == right),
            _ => unreachable!(),
        })
    }

    fn visit_call(
        &self,
        callee: &Token,
        paren: &Expr,
        arguments: &[Expr],
    ) -> anyhow::Result<Object> {
        todo!();
    }

    fn visit_get(&self, object: &Expr, name: &Token) -> anyhow::Result<Object> {
        todo!();
    }

    fn visit_grouping(&self, expression: &Expr) -> anyhow::Result<Object> {
        self.evaluate(expression)
    }

    fn visit_literal(&self, object: &Object) -> anyhow::Result<Object> {
        Ok(object.clone())
    }

    fn visit_logical(&self, left: &Expr, operator: &Token, right: &Expr) -> anyhow::Result<Object> {
        todo!();
    }

    fn visit_set(&self, object: &Expr, token: &Token, value: &Expr) -> anyhow::Result<Object> {
        todo!();
    }

    fn visit_super(&self, keyword: &Token, method: &Token) -> anyhow::Result<Object> {
        todo!();
    }

    fn visit_this(&self, keyword: &Token) -> anyhow::Result<Object> {
        todo!();
    }

    fn visit_unary(&self, operator: &Token, right: &Expr) -> anyhow::Result<Object> {
        let right = self.evaluate(right)?;

        Ok(match operator.token_type {
            TokenType::Bang => Object::Boolean(!right.is_truthy()),
            TokenType::Minus => Object::Number(-right.try_into()?),
            _ => unreachable!(),
        })
    }

    fn visit_variable(&self, name: &Token) -> anyhow::Result<Object> {
        todo!();
    }
}
