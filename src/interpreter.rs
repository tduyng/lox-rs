use crate::parser::Expr;

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&mut self, expr: Expr) -> String {
        match expr {
            Expr::Number(n) => n.to_string(),
            Expr::Boolean(b) => b.to_string(),
            Expr::Nil => "nil".to_string(),
        }
    }
}
