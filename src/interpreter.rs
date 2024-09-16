use crate::{parser::Expr, token::TokenType};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&mut self, expr: Expr) -> String {
        match expr {
            Expr::String(s) => s,
            Expr::Number(n) => round_to_string(n, 2),
            Expr::Boolean(b) => b.to_string(),
            Expr::Nil => "nil".to_string(),
            Expr::Unary { operator, right } => {
                let right_val = self.evaluate(*right);
                match operator.token_type {
                    TokenType::Minus => {
                        if let Ok(num) = right_val.parse::<f64>() {
                            return round_to_string(-num, 2);
                        }
                        panic!("Operand must be a number.");
                    }
                    TokenType::Bang => {
                        let is_truthy = self.is_truthy(&right_val);
                        (!is_truthy).to_string()
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
                self.handle_binary_op(&left_val, &operator.token_type, &right_val)
            }
        }
    }

    fn handle_binary_op(&self, left: &str, operator: &TokenType, right: &str) -> String {
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
            _ => "".to_string(),
        }
    }

    fn handle_plus(&self, left: &str, right: &str) -> String {
        if let (Ok(left_num), Ok(right_num)) = (left.parse::<f64>(), right.parse::<f64>()) {
            return round_to_string(left_num + right_num, 2);
        }
        format!("{}{}", left, right)
    }

    fn handle_minus(&self, left: &str, right: &str) -> String {
        if let (Ok(left_num), Ok(right_num)) = (left.parse::<f64>(), right.parse::<f64>()) {
            return round_to_string(left_num - right_num, 2);
        }
        panic!("Operands must be numbers.");
    }

    fn handle_divide(&self, left: &str, right: &str) -> String {
        if let (Ok(left_num), Ok(right_num)) = (left.parse::<f64>(), right.parse::<f64>()) {
            if right_num == 0.0 {
                panic!("Division by zero.");
            }
            return round_to_string(left_num / right_num, 2);
        }
        panic!("Operands must be numbers.");
    }

    fn handle_multiply(&self, left: &str, right: &str) -> String {
        if let (Ok(left_num), Ok(right_num)) = (left.parse::<f64>(), right.parse::<f64>()) {
            return round_to_string(left_num * right_num, 2);
        }
        panic!("Operands must be numbers.");
    }

    fn handle_greater(&self, left: &str, right: &str) -> String {
        if let (Ok(left_num), Ok(right_num)) = (left.parse::<f64>(), right.parse::<f64>()) {
            return (left_num > right_num).to_string();
        }
        panic!("Operands must be numbers.");
    }

    fn handle_greater_equal(&self, left: &str, right: &str) -> String {
        if let (Ok(left_num), Ok(right_num)) = (left.parse::<f64>(), right.parse::<f64>()) {
            return (left_num >= right_num).to_string();
        }
        panic!("Operands must be numbers.");
    }

    fn handle_less(&self, left: &str, right: &str) -> String {
        if let (Ok(left_num), Ok(right_num)) = (left.parse::<f64>(), right.parse::<f64>()) {
            return (left_num < right_num).to_string();
        }
        panic!("Operands must be numbers.");
    }

    fn handle_less_equal(&self, left: &str, right: &str) -> String {
        if let (Ok(left_num), Ok(right_num)) = (left.parse::<f64>(), right.parse::<f64>()) {
            return (left_num <= right_num).to_string();
        }
        panic!("Operands must be numbers.");
    }

    fn handle_equal_equal(&self, left: &str, right: &str) -> String {
        (left == right).to_string()
    }

    fn handle_bang_equal(&self, left: &str, right: &str) -> String {
        (left != right).to_string()
    }

    fn is_truthy(&self, value: &str) -> bool {
        value != "false" && value != "nil"
    }
}

pub fn round_to_string(value: f64, precision: usize) -> String {
    let multiplier = 10_f64.powi(precision as i32);
    let rounded_value = (value * multiplier).round() / multiplier;
    format!("{:.1$}", rounded_value, precision)
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}
