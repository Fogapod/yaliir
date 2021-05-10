use crate::environment::Environment;
use crate::errors::RuntimeError;
use crate::expression::{self, Expr};
use crate::object::Object;
use crate::statement::{self, Stmt};
use crate::token::{Token, TokenType};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(None),
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> anyhow::Result<()> {
        for statement in statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> anyhow::Result<Object> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Stmt) -> anyhow::Result<()> {
        stmt.accept(self)
    }

    fn execute_block(
        &mut self,
        statements: &[Stmt],
        environment: &Environment,
    ) -> anyhow::Result<()> {
        let previous = self.environment.clone();

        self.environment = environment.clone();

        for statement in statements {
            if let Err(e) = self.execute(statement) {
                self.environment = previous;

                anyhow::bail!(e);
            }
        }

        Ok(())
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

    fn operands_cmp_ne(left: &Object, right: &Object, _operator: &Token) -> Object {
        Object::Boolean(left != right)
    }

    fn operands_cmp_eq(left: &Object, right: &Object, _operator: &Token) -> Object {
        Object::Boolean(left == right)
    }
}

impl expression::Visitor<anyhow::Result<Object>> for Interpreter {
    fn visit_assign(&mut self, name: &Token, value: &Expr) -> anyhow::Result<Object> {
        let value = self.evaluate(value)?;

        self.environment.assign(&name, &value)?;

        Ok(value)
    }

    fn visit_binary(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> anyhow::Result<Object> {
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
            TokenType::BangEqual => Self::operands_cmp_ne(left, right, operator),
            TokenType::EqualEqual => Self::operands_cmp_eq(left, right, operator),
            _ => unreachable!(),
        })
    }

    fn visit_call(
        &mut self,
        callee: &Token,
        paren: &Expr,
        arguments: &[Expr],
    ) -> anyhow::Result<Object> {
        todo!();
    }

    fn visit_get(&mut self, object: &Expr, name: &Token) -> anyhow::Result<Object> {
        todo!();
    }

    fn visit_grouping(&mut self, expression: &Expr) -> anyhow::Result<Object> {
        self.evaluate(expression)
    }

    fn visit_literal(&mut self, object: &Object) -> anyhow::Result<Object> {
        Ok(object.clone())
    }

    fn visit_logical(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> anyhow::Result<Object> {
        let left = self.evaluate(left)?;

        if operator.token_type == TokenType::Or {
            if left.is_truthy() {
                return Ok(left);
            }
        } else if !left.is_truthy() {
            return Ok(left);
        }

        self.evaluate(right)
    }

    fn visit_set(&mut self, object: &Expr, token: &Token, value: &Expr) -> anyhow::Result<Object> {
        todo!();
    }

    fn visit_super(&mut self, keyword: &Token, method: &Token) -> anyhow::Result<Object> {
        todo!();
    }

    fn visit_this(&mut self, keyword: &Token) -> anyhow::Result<Object> {
        todo!();
    }

    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> anyhow::Result<Object> {
        let right = self.evaluate(right)?;

        Ok(match operator.token_type {
            TokenType::Bang => Object::Boolean(!right.is_truthy()),
            TokenType::Minus => Object::Number(-Self::operand_into_number(operator, &right)?),
            _ => unreachable!(),
        })
    }

    fn visit_variable(&mut self, name: &Token) -> anyhow::Result<Object> {
        self.environment.get(name)
    }
}

impl statement::Visitor<anyhow::Result<()>> for Interpreter {
    fn visit_block(&mut self, statements: &[Stmt]) -> anyhow::Result<()> {
        self.execute_block(
            statements,
            &Environment::new(Some(self.environment.clone())),
        )
    }

    fn visit_expression(&mut self, value: &Expr) -> anyhow::Result<()> {
        self.evaluate(value).map(|_| {})
    }

    fn visit_if(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Box<Stmt>>,
    ) -> anyhow::Result<()> {
        if self.evaluate(condition)?.is_truthy() {
            self.execute(then_branch)?;
        } else if let Some(else_branch) = else_branch {
            self.execute(else_branch)?;
        }

        Ok(())
    }

    fn visit_print(&mut self, value: &Expr) -> anyhow::Result<()> {
        let value = self.evaluate(value)?;

        println!("{}", value);

        Ok(())
    }

    fn visit_var(&mut self, name: &Token, initializer: &Option<Expr>) -> anyhow::Result<()> {
        let mut value = Object::Null;

        if let Some(initializer) = initializer {
            value = self.evaluate(initializer)?;
        }

        self.environment.define(&name.lexeme, &value);

        Ok(())
    }

    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> anyhow::Result<()> {
        while self.evaluate(condition)?.is_truthy() {
            self.execute(body)?;
        }

        Ok(())
    }
}
