use std::collections::HashMap;

use crate::errors::RuntimeError;
use crate::object::Object;
use crate::token::Token;

#[derive(Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new(enclosing: Option<Environment>) -> Self {
        Self {
            enclosing: enclosing.map(Box::new),
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> anyhow::Result<Object> {
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => {
                if let Some(enclosing) = &self.enclosing {
                    return enclosing.get(name);
                }

                anyhow::bail!(RuntimeError {
                    token: name.clone(),
                    message: format!("Undefined variable '{}'.", name.lexeme)
                });
            }
        }
    }

    pub fn assign(&mut self, name: &Token, value: &Object) -> anyhow::Result<()> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());

            return Ok(());
        }

        if let Some(enclosing) = &mut self.enclosing {
            return enclosing.assign(name, value);
        }

        anyhow::bail!(RuntimeError {
            token: name.clone(),
            message: format!("Undefined variable '{}'.", name.lexeme)
        });
    }

    pub fn define(&mut self, name: &str, value: &Object) {
        self.values.insert(name.to_string(), value.clone());
    }
}
