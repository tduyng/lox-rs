use crate::{ast::Expr, error::LoxError};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    pub values: HashMap<String, Expr>,
    pub parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Environment) -> Self {
        Self {
            values: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn define(&mut self, name: String, value: Expr) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Expr> {
        self.values
            .get(name)
            .or_else(|| self.parent.as_deref().and_then(|parent| parent.get(name)))
    }

    #[allow(clippy::map_entry)]
    pub fn assign(&mut self, name: String, value: Expr, line: usize) -> Result<(), LoxError> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            Ok(())
        } else if let Some(parent) = &mut self.parent {
            parent.assign(name, value, line)
        } else {
            Err(LoxError::new("Undefined variable", Some(line)))
        }
    }
}
