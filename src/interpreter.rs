use crate::{parser::Expr, token::TokenType};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&mut self, expr: Expr) -> Expr {
        match expr {
            Expr::String(s) => Expr::String(s),
            Expr::Number(n) => Expr::Number(n),
            Expr::Boolean(b) => Expr::Boolean(b),
            Expr::Nil => Expr::Nil,
            Expr::Unary { operator, right } => {
                let right_val = self.evaluate(*right);
                match operator.token_type {
                    TokenType::Minus => {
                        if let Expr::Number(n) = right_val {
                            return Expr::Number(-n);
                        }
                        panic!("Operand must be a number.");
                    }
                    TokenType::Bang => {
                        let is_truthy = self.is_truthy(&right_val);
                        return Expr::Boolean(!is_truthy);
                    }
                    _ => panic!("Unknown unary operator."),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate(*left);
                let right_val = self.evaluate(*right);
                self.handle_binary_op(left_val, &operator.token_type, right_val)
            }
        }
    }

    fn handle_binary_op(&self, left: Expr, operator: &TokenType, right: Expr) -> Expr {
        match operator {
            TokenType::Plus => self.handle_plus(left, right),
            TokenType::Minus => self.handle_minus(left, right),
            TokenType::Slash => self.handle_divide(left, right),
            TokenType::Star => self.handle_multiply(left, right),
            TokenType::Greater => self.handle_greater(left, right),
            TokenType::GreaterEqual => self.handle_greater_equal(left, right),
            TokenType::Less => self.handle_less(left, right),
            TokenType::LessEqual => self.handle_less_equal(left, right),
            TokenType::EqualEqual => self.handle_equal_equal(left, right),
            TokenType::BangEqual => self.handle_bang_equal(left, right),
            _ => Expr::Nil,
        }
    }

    fn handle_plus(&self, left: Expr, right: Expr) -> Expr {
        match (left, right) {
            (Expr::Number(l), Expr::Number(r)) => Expr::Number(round(l + r, 2)),
            (Expr::String(l), Expr::String(r)) => Expr::String(format!("{}{}", l, r)),
            _ => panic!("Operands must be two numbers or two strings."),
        }
    }

    fn handle_minus(&self, left: Expr, right: Expr) -> Expr {
        if let (Expr::Number(l), Expr::Number(r)) = (left, right) {
            Expr::Number(round(l - r, 2))
        } else {
            panic!("Operands must be numbers.");
        }
    }

    fn handle_divide(&self, left: Expr, right: Expr) -> Expr {
        if let (Expr::Number(l), Expr::Number(r)) = (left, right) {
            if r == 0.0 {
                panic!("Division by zero.");
            }
            Expr::Number(round(l / r, 2))
        } else {
            panic!("Operands must be numbers.");
        }
    }

    fn handle_multiply(&self, left: Expr, right: Expr) -> Expr {
        if let (Expr::Number(l), Expr::Number(r)) = (left, right) {
            Expr::Number(round(l * r, 2))
        } else {
            panic!("Operands must be numbers.");
        }
    }

    fn handle_greater(&self, left: Expr, right: Expr) -> Expr {
        if let (Expr::Number(l), Expr::Number(r)) = (left, right) {
            Expr::Boolean(l > r)
        } else {
            panic!("Operands must be numbers.");
        }
    }

    fn handle_greater_equal(&self, left: Expr, right: Expr) -> Expr {
        if let (Expr::Number(l), Expr::Number(r)) = (left, right) {
            Expr::Boolean(l >= r)
        } else {
            panic!("Operands must be numbers.");
        }
    }

    fn handle_less(&self, left: Expr, right: Expr) -> Expr {
        if let (Expr::Number(l), Expr::Number(r)) = (left, right) {
            Expr::Boolean(l < r)
        } else {
            panic!("Operands must be numbers.");
        }
    }

    fn handle_less_equal(&self, left: Expr, right: Expr) -> Expr {
        if let (Expr::Number(l), Expr::Number(r)) = (left, right) {
            Expr::Boolean(l <= r)
        } else {
            panic!("Operands must be numbers.");
        }
    }

    fn handle_equal_equal(&self, left: Expr, right: Expr) -> Expr {
        Expr::Boolean(left == right)
    }

    fn handle_bang_equal(&self, left: Expr, right: Expr) -> Expr {
        Expr::Boolean(left != right)
    }

    fn is_truthy(&self, value: &Expr) -> bool {
        match value {
            Expr::Boolean(b) => *b,
            Expr::Nil => false,
            _ => true,
        }
    }
}

fn round(value: f64, precision: usize) -> f64 {
    let multiplier = 10_f64.powi(precision as i32);
    (value * multiplier).round() / multiplier
}
