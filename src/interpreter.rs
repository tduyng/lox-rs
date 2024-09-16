use crate::{parser::Expr, token::TokenType};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&mut self, expr: Expr) -> String {
        match expr {
            Expr::String(s) => s,
            Expr::Number(n) => n.to_string(),
            Expr::Boolean(b) => b.to_string(),
            Expr::Nil => "nil".to_string(),
            Expr::Unary { operator, right } => {
                let right_val = self.evaluate(*right);
                match operator.token_type {
                    TokenType::Minus => {
                        if let Ok(num) = right_val.parse::<f64>() {
                            return (-num).to_string();
                        }
                        panic!("Operand must be a number.");
                    }
                    TokenType::Bang => {
                        let is_truthy = self.is_truthy(&right_val);
                        return (!is_truthy).to_string();
                    }
                    _ => panic!("Unknown unary operator."),
                }
            }
        }
    }

    fn is_truthy(&self, value: &str) -> bool {
        value != "false" && value != "nil"
    }
}
