use crate::{
    ast::{Expr, Stmt},
    token::TokenType,
};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn interpret(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Print(expr) => {
                let value = self.evaluate(expr);
                self.print_value(value);
            }
            Stmt::Expression(expr) => {
                self.evaluate(expr);
            }
        }
    }

    pub fn evaluate(&mut self, expr: Expr) -> Expr {
        match expr {
            Expr::String(s) => Expr::String(s),
            Expr::Number(n) => Expr::Number(n),
            Expr::Boolean(b) => Expr::Boolean(b),
            Expr::Nil => Expr::Nil,
            Expr::Unary { operator, right } => {
                let right_val = self.evaluate(*right);
                let line_number = operator.line;
                match operator.token_type {
                    TokenType::Minus => {
                        if let Expr::Number(n) = right_val {
                            return Expr::Number(-n);
                        }
                        eprintln!("Operand must be a number.\n[line {}]", line_number);
                        std::process::exit(70);
                    }
                    TokenType::Bang => {
                        let is_truthy = self.is_truthy(&right_val);
                        Expr::Boolean(!is_truthy)
                    }
                    _ => {
                        eprintln!("Unknown unary operator.\n[line {}]", line_number);
                        std::process::exit(70);
                    }
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate(*left);
                let right_val = self.evaluate(*right);
                let line = operator.line;
                self.handle_binary_op(left_val, &operator.token_type, right_val, line)
            }
            Expr::Grouping(inner_expr) => self.evaluate(*inner_expr),
        }
    }

    pub fn print_value(&self, value: Expr) {
        match value {
            Expr::String(s) => println!("{}", s),
            Expr::Number(n) => println!("{}", n),
            Expr::Boolean(b) => println!("{}", b),
            Expr::Nil => println!("nil"),
            Expr::Unary { operator, right } => {
                println!("({} {})", operator.lexeme, right)
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => println!("({} {} {})", operator.lexeme, left, right),
            Expr::Grouping(expr) => println!("(group {})", expr),
        }
    }

    fn handle_binary_op(&self, left: Expr, operator: &TokenType, right: Expr, line: usize) -> Expr {
        match operator {
            TokenType::Plus => self.handle_plus(left, right, line),
            TokenType::Minus => self.handle_minus(left, right, line),
            TokenType::Slash => self.handle_divide(left, right, line),
            TokenType::Star => self.handle_multiply(left, right, line),
            TokenType::Greater => self.handle_greater(left, right, line),
            TokenType::GreaterEqual => self.handle_greater_equal(left, right, line),
            TokenType::Less => self.handle_less(left, right, line),
            TokenType::LessEqual => self.handle_less_equal(left, right, line),
            TokenType::EqualEqual => self.handle_equal_equal(left, right),
            TokenType::BangEqual => self.handle_bang_equal(left, right),
            _ => Expr::Nil,
        }
    }

    fn handle_plus(&self, left: Expr, right: Expr, line: usize) -> Expr {
        match (left, right) {
            (Expr::Number(l), Expr::Number(r)) => Expr::Number(l + r),
            (Expr::String(l), Expr::String(r)) => Expr::String(format!("{}{}", l, r)),
            _ => {
                eprintln!(
                    "Operands must be two numbers or two strings.\n[line {}]",
                    line
                );
                std::process::exit(70);
            }
        }
    }

    fn handle_minus(&self, left: Expr, right: Expr, line: usize) -> Expr {
        if let (Expr::Number(l), Expr::Number(r)) = (left, right) {
            Expr::Number(l - r)
        } else {
            eprintln!("Operands must be numbers.\n[line {}]", line);
            std::process::exit(70);
        }
    }

    fn handle_divide(&self, left: Expr, right: Expr, line: usize) -> Expr {
        if let (Expr::Number(l), Expr::Number(r)) = (left, right) {
            if r == 0.0 {
                eprintln!("Division by zero.\n[line {}]", line);
                std::process::exit(70);
            }
            Expr::Number(l / r)
        } else {
            eprintln!("Operands must be numbers.\n[line {}]", line);
            std::process::exit(70);
        }
    }

    fn handle_multiply(&self, left: Expr, right: Expr, line: usize) -> Expr {
        if let (Expr::Number(l), Expr::Number(r)) = (left, right) {
            Expr::Number(l * r)
        } else {
            eprintln!("Operands must be numbers.\n[line {}]", line);
            std::process::exit(70);
        }
    }

    fn handle_greater(&self, left: Expr, right: Expr, line: usize) -> Expr {
        if let (Expr::Number(l), Expr::Number(r)) = (left, right) {
            Expr::Boolean(l > r)
        } else {
            eprintln!("Operands must be numbers.\n[line {}]", line);
            std::process::exit(70);
        }
    }

    fn handle_greater_equal(&self, left: Expr, right: Expr, line: usize) -> Expr {
        if let (Expr::Number(l), Expr::Number(r)) = (left, right) {
            Expr::Boolean(l >= r)
        } else {
            eprintln!("Operands must be numbers.\n[line {}]", line);
            std::process::exit(70);
        }
    }

    fn handle_less(&self, left: Expr, right: Expr, line: usize) -> Expr {
        if let (Expr::Number(l), Expr::Number(r)) = (left, right) {
            Expr::Boolean(l < r)
        } else {
            eprintln!("Operands must be numbers.\n[line {}]", line);
            std::process::exit(70);
        }
    }

    fn handle_less_equal(&self, left: Expr, right: Expr, line: usize) -> Expr {
        if let (Expr::Number(l), Expr::Number(r)) = (left, right) {
            Expr::Boolean(l <= r)
        } else {
            eprintln!("Operands must be numbers.\n[line {}]", line);
            std::process::exit(70);
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
