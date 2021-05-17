use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::time::SystemTime;

use crate::callable::Callable;
use crate::environment::{Environment, SharedEnv};
use crate::error::LoxError;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::statement::Stmt;
use crate::token::Token;

#[derive(Clone)]
pub enum Function {
    Native {
        arity: usize,
        function: fn(
            function: &Self,
            interpreter: &Interpreter,
            arguments: &[Object],
        ) -> Result<Object, LoxError>,
    },
    User {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
        closure: SharedEnv,
    },
}

impl Callable for Function {
    fn arity(&self) -> usize {
        match self {
            Function::Native { arity, .. } => *arity,
            Function::User { params, .. } => params.len(),
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Object],
    ) -> Result<Object, LoxError> {
        match self {
            Function::Native { function, .. } => function(self, interpreter, arguments),
            Function::User {
                params,
                body,
                closure,
                ..
            } => {
                let mut environment = Rc::new(RefCell::new(Environment::from(closure)));

                for (param, argument) in params.iter().zip(arguments) {
                    Rc::get_mut(&mut environment)
                        .unwrap()
                        .get_mut()
                        .define(&param.lexeme, argument);
                }

                if let Err(err) = interpreter.execute_block(&body, &environment) {
                    return match err {
                        LoxError::Return(value) => Ok(value),
                        err => Err(err),
                    };
                }

                Ok(Object::Null)
            }
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Function::Native { .. } => write!(f, "<native fn>"),
            Function::User { name, .. } => write!(f, "<fn {}>", name.lexeme),
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Function::Native { arity, .. } => f
                .debug_struct("Function::Native")
                .field("arity", arity)
                .finish(),
            Function::User {
                name,
                params,
                body,
                closure,
            } => f
                .debug_struct("Function::User")
                .field("name", name)
                .field("params", params)
                .field("body", body)
                .field("closure", closure)
                .finish(),
        }
    }
}

impl Function {
    // Native functions
    pub fn new_native_fn_clock() -> Self {
        Function::Native {
            arity: 0,
            function: |_, _, _| {
                let start = SystemTime::now();
                Ok(Object::Number(
                    start
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .expect("unable to get system time")
                        .as_secs_f64(),
                ))
            },
        }
    }
}
