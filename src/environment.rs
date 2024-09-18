use crate::{ast::Expr, error::LoxError};
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, Expr>,
}

#[allow(dead_code)]
impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Expr) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Expr> {
        self.values.get(name)
    }

    pub fn assign(&mut self, name: String, value: Expr, line: usize) -> Result<(), LoxError> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            Ok(())
        } else {
            Err(LoxError::new("Undefined variable", line))
        }
    }
}
