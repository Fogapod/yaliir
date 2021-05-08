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

    fn operand_into_number(operator: &Token, operand: &Object) -> anyhow::Result<f64> {
        match operand {
            Object::Number(value) => Ok(*value),
            _ => anyhow::bail!(RuntimeError {
                token: operator.clone(),
                message: "Operand must be a number.".to_string()
            }),
        }
    }

    fn operands_subtract(
        left: &Object,
        right: &Object,
        operator: &Token,
    ) -> anyhow::Result<Object> {
        Ok(Object::Number(
            Self::operand_into_number(operator, &left)?
                - Self::operand_into_number(operator, &right)?,
        ))
    }

    fn operands_divide(left: &Object, right: &Object, operator: &Token) -> anyhow::Result<Object> {
        Ok(Object::Number(
            Self::operand_into_number(operator, &left)?
                / Self::operand_into_number(operator, &right)?,
        ))
    }

    fn operands_multiply(
        left: &Object,
        right: &Object,
        operator: &Token,
    ) -> anyhow::Result<Object> {
        Ok(Object::Number(
            Self::operand_into_number(operator, &left)?
                * Self::operand_into_number(operator, &right)?,
        ))
    }

    fn operands_add(left: &Object, right: &Object, operator: &Token) -> anyhow::Result<Object> {
        Ok(match (&left, &right) {
            (Object::Number(number1), Object::Number(number2)) => Object::Number(number1 + number2),
            (Object::String(string1), Object::String(string2)) => {
                Object::String(string1.to_owned() + string2)
            }
            _ => anyhow::bail!(RuntimeError {
                token: operator.clone(),
                message: "Operands must be two numbers or two strings.".to_string()
            }),
        })
    }

    fn operands_cmp_gt(left: &Object, right: &Object, operator: &Token) -> anyhow::Result<Object> {
        Ok(Object::Boolean(
            Self::operand_into_number(operator, &left)?
                > Self::operand_into_number(operator, &right)?,
        ))
    }

    fn operands_cmp_ge(left: &Object, right: &Object, operator: &Token) -> anyhow::Result<Object> {
        Ok(Object::Boolean(
            Self::operand_into_number(operator, &left)?
                >= Self::operand_into_number(operator, &right)?,
        ))
    }

    fn operands_cmp_lt(left: &Object, right: &Object, operator: &Token) -> anyhow::Result<Object> {
        Ok(Object::Boolean(
            Self::operand_into_number(operator, &left)?
                < Self::operand_into_number(operator, &right)?,
        ))
    }

    fn operands_cmp_le(left: &Object, right: &Object, operator: &Token) -> anyhow::Result<Object> {
        Ok(Object::Boolean(
            Self::operand_into_number(operator, &left)?
                <= Self::operand_into_number(operator, &right)?,
        ))
    }

    fn operands_cmp_ne(left: &Object, right: &Object, _operator: &Token) -> anyhow::Result<Object> {
        Ok(Object::Boolean(left == right))
    }

    fn operands_cmp_eq(left: &Object, right: &Object, _operator: &Token) -> anyhow::Result<Object> {
        Ok(Object::Boolean(left != right))
    }
}

impl Visitor<anyhow::Result<Object>> for Interpreter {
    fn visit_assign(&self, name: &Token, value: &Expr) -> anyhow::Result<Object> {
        todo!();
    }

    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> anyhow::Result<Object> {
        let left = &self.evaluate(left)?;
        let right = &self.evaluate(right)?;

        Ok(match operator.token_type {
            TokenType::Minus => Self::operands_subtract(left, right, operator)?,
            TokenType::Slash => Self::operands_divide(left, right, operator)?,
            TokenType::Star => Self::operands_multiply(&left, right, operator)?,
            TokenType::Plus => Self::operands_add(left, right, operator)?,
            TokenType::Greater => Self::operands_cmp_gt(left, right, operator)?,
            TokenType::GreaterEqual => Self::operands_cmp_ge(left, right, operator)?,
            TokenType::Less => Self::operands_cmp_lt(left, right, operator)?,
            TokenType::LessEqual => Self::operands_cmp_le(left, right, operator)?,
            TokenType::BangEqual => Self::operands_cmp_ne(left, right, operator)?,
            TokenType::EqualEqual => Self::operands_cmp_eq(left, right, operator)?,
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
            TokenType::Minus => Object::Number(-Self::operand_into_number(operator, &right)?),
            _ => unreachable!(),
        })
    }

    fn visit_variable(&self, name: &Token) -> anyhow::Result<Object> {
        todo!();
    }
}
