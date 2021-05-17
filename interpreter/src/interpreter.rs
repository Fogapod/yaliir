use std::cell::RefCell;
use std::convert::From;
use std::rc::Rc;

use crate::callable::Callable;
use crate::environment::{Environment, SharedEnv};
use crate::error::LoxError;
use crate::expression::{self, Expr};
use crate::function::Function;
use crate::object::Object;
use crate::statement::{self, Stmt};
use crate::token::{Token, TokenType};

pub struct Interpreter {
    globals: SharedEnv,
    environment: SharedEnv,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Environment::new();

        globals.define("clock", &Object::Callable(Function::new_native_fn_clock()));

        let globals_shared = SharedEnv::from(globals);

        Self {
            globals: Rc::clone(&globals_shared),
            environment: globals_shared,
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), LoxError> {
        for statement in statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), LoxError> {
        stmt.accept(self)
    }

    pub(crate) fn execute_block(
        &mut self,
        statements: &[Stmt],
        environment: &SharedEnv,
    ) -> Result<(), LoxError> {
        let previous = self.environment.clone();

        self.environment = environment.clone();

        let mut guarded = || -> Result<(), LoxError> {
            for statement in statements {
                self.execute(statement)?;
            }

            Ok(())
        };

        let result = guarded();

        self.environment = previous;

        result
    }

    fn operand_into_number(operator: &Token, operand: &Object) -> Result<f64, LoxError> {
        match operand {
            Object::Number(value) => Ok(*value),
            _ => Err(LoxError::Runtime {
                token: operator.clone(),
                message: "Operand must be a number.".to_owned(),
            }),
        }
    }

    fn operands_subtract(
        left: &Object,
        right: &Object,
        operator: &Token,
    ) -> Result<Object, LoxError> {
        Ok(Object::Number(
            Self::operand_into_number(operator, &left)?
                - Self::operand_into_number(operator, &right)?,
        ))
    }

    fn operands_divide(
        left: &Object,
        right: &Object,
        operator: &Token,
    ) -> Result<Object, LoxError> {
        Ok(Object::Number(
            Self::operand_into_number(operator, &left)?
                / Self::operand_into_number(operator, &right)?,
        ))
    }

    fn operands_multiply(
        left: &Object,
        right: &Object,
        operator: &Token,
    ) -> Result<Object, LoxError> {
        Ok(Object::Number(
            Self::operand_into_number(operator, &left)?
                * Self::operand_into_number(operator, &right)?,
        ))
    }

    fn operands_add(left: &Object, right: &Object, operator: &Token) -> Result<Object, LoxError> {
        Ok(match (&left, &right) {
            (Object::Number(number1), Object::Number(number2)) => Object::Number(number1 + number2),
            (Object::String(string1), Object::String(string2)) => {
                Object::String(string1.to_owned() + string2)
            }
            _ => {
                return Err(LoxError::Runtime {
                    token: operator.clone(),
                    message: "Operands must be two numbers or two strings.".to_owned(),
                })
            }
        })
    }

    fn operands_cmp_gt(
        left: &Object,
        right: &Object,
        operator: &Token,
    ) -> Result<Object, LoxError> {
        Ok(Object::Boolean(
            Self::operand_into_number(operator, &left)?
                > Self::operand_into_number(operator, &right)?,
        ))
    }

    fn operands_cmp_ge(
        left: &Object,
        right: &Object,
        operator: &Token,
    ) -> Result<Object, LoxError> {
        Ok(Object::Boolean(
            Self::operand_into_number(operator, &left)?
                >= Self::operand_into_number(operator, &right)?,
        ))
    }

    fn operands_cmp_lt(
        left: &Object,
        right: &Object,
        operator: &Token,
    ) -> Result<Object, LoxError> {
        Ok(Object::Boolean(
            Self::operand_into_number(operator, &left)?
                < Self::operand_into_number(operator, &right)?,
        ))
    }

    fn operands_cmp_le(
        left: &Object,
        right: &Object,
        operator: &Token,
    ) -> Result<Object, LoxError> {
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

impl expression::Visitor<Result<Object, LoxError>> for Interpreter {
    fn visit_assign(&mut self, name: &Token, value: &Expr) -> Result<Object, LoxError> {
        let value = self.evaluate(value)?;

        self.environment.borrow_mut().assign(&name, &value)?;

        Ok(value)
    }

    fn visit_binary(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Object, LoxError> {
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
        callee: &Expr,
        paren: &Token,
        arguments: &[Expr],
    ) -> Result<Object, LoxError> {
        let callee = self.evaluate(callee)?;

        let mut args = Vec::with_capacity(arguments.len());

        for argument in arguments {
            args.push(self.evaluate(argument)?);
        }

        let function = match callee {
            Object::Callable(function) => function,
            e => {
                println!("{:?}", e);
                return Err(LoxError::Runtime {
                    token: paren.clone(),
                    message: "Can only call functions and classes.".to_owned(),
                });
            }
        };

        if args.len() != function.arity() as usize {
            return Err(LoxError::Runtime {
                token: paren.clone(),
                message: format!(
                    "Expected {} arguments but got {}.",
                    function.arity(),
                    args.len()
                ),
            });
        }

        function.call(self, &args)
    }

    fn visit_get(&mut self, object: &Expr, name: &Token) -> Result<Object, LoxError> {
        todo!();
    }

    fn visit_grouping(&mut self, expression: &Expr) -> Result<Object, LoxError> {
        self.evaluate(expression)
    }

    fn visit_literal(&mut self, object: &Object) -> Result<Object, LoxError> {
        Ok(object.clone())
    }

    fn visit_logical(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Object, LoxError> {
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

    fn visit_set(
        &mut self,
        object: &Expr,
        token: &Token,
        value: &Expr,
    ) -> Result<Object, LoxError> {
        todo!();
    }

    fn visit_super(&mut self, keyword: &Token, method: &Token) -> Result<Object, LoxError> {
        todo!();
    }

    fn visit_this(&mut self, keyword: &Token) -> Result<Object, LoxError> {
        todo!();
    }

    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Result<Object, LoxError> {
        let right = self.evaluate(right)?;

        Ok(match operator.token_type {
            TokenType::Bang => Object::Boolean(!right.is_truthy()),
            TokenType::Minus => Object::Number(-Self::operand_into_number(operator, &right)?),
            _ => unreachable!(),
        })
    }

    fn visit_variable(&mut self, name: &Token) -> Result<Object, LoxError> {
        self.environment.borrow().get(name)
    }
}

impl statement::Visitor<Result<(), LoxError>> for Interpreter {
    fn visit_block(&mut self, statements: &[Stmt]) -> Result<(), LoxError> {
        self.execute_block(
            statements,
            &Rc::new(RefCell::new(Environment::from(&self.environment))),
        )
    }

    fn visit_expression(&mut self, value: &Expr) -> Result<(), LoxError> {
        self.evaluate(value).map(|_| {})
    }

    fn visit_function(
        &mut self,
        name: &Token,
        params: &[Token],
        body: &[Stmt],
    ) -> Result<(), LoxError> {
        let function = Function::User {
            name: name.clone(),
            params: params.to_vec(),
            body: body.to_vec(),
            closure: Rc::clone(&self.environment),
        };

        self.environment
            .borrow_mut()
            .define(&name.lexeme, &Object::Callable(function));

        Ok(())
    }

    fn visit_if(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Box<Stmt>>,
    ) -> Result<(), LoxError> {
        if self.evaluate(condition)?.is_truthy() {
            self.execute(then_branch)?;
        } else if let Some(else_branch) = else_branch {
            self.execute(else_branch)?;
        }

        Ok(())
    }

    fn visit_print(&mut self, value: &Expr) -> Result<(), LoxError> {
        let value = self.evaluate(value)?;

        println!("{}", value);

        Ok(())
    }

    fn visit_return(&mut self, _keyword: &Token, value: &Option<Expr>) -> Result<(), LoxError> {
        let value = if let Some(v) = value {
            self.evaluate(v)?
        } else {
            Object::Null
        };

        Err(LoxError::Return(value))
    }

    fn visit_var(&mut self, name: &Token, initializer: &Option<Expr>) -> Result<(), LoxError> {
        let mut value = Object::Null;

        if let Some(initializer) = initializer {
            value = self.evaluate(initializer)?;
        }

        self.environment.borrow_mut().define(&name.lexeme, &value);

        Ok(())
    }

    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> Result<(), LoxError> {
        while self.evaluate(condition)?.is_truthy() {
            self.execute(body)?;
        }

        Ok(())
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
