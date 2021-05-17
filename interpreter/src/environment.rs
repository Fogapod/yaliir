use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::{From, Into};
use std::rc::Rc;

use crate::error::LoxError;
use crate::object::Object;
use crate::token::Token;

pub type SharedEnv = Rc<RefCell<Environment>>;

#[derive(Clone, Debug)]
pub struct Environment {
    // TODO: hide this implementation detail and share `values` field internally?
    enclosing: Option<SharedEnv>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new_shared() -> SharedEnv {
        Environment::new().into()
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxError> {
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => {
                if let Some(enclosing) = &self.enclosing {
                    return enclosing.borrow().get(name);
                }

                return Err(LoxError::Runtime {
                    token: name.clone(),
                    message: format!("Undefined variable '{}'.", name.lexeme),
                });
            }
        }
    }

    pub fn assign(&mut self, name: &Token, value: &Object) -> Result<(), LoxError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());

            return Ok(());
        }

        if let Some(enclosing) = &mut self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
        }

        return Err(LoxError::Runtime {
            token: name.clone(),
            message: format!("Undefined variable '{}'.", name.lexeme),
        });
    }

    pub fn define(&mut self, name: &str, value: &Object) {
        self.values.insert(name.to_owned(), value.clone());
    }
}

impl From<&SharedEnv> for Environment {
    fn from(existing: &SharedEnv) -> Self {
        Self {
            enclosing: Some(Rc::clone(existing)),
            values: HashMap::new(),
        }
    }
}

impl From<Environment> for SharedEnv {
    fn from(existing: Environment) -> Self {
        Rc::new(RefCell::new(existing))
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
