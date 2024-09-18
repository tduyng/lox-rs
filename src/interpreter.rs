use crate::{
    ast::{Expr, Stmt},
    environment::Environment,
    error::LoxError,
    token::TokenType,
};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), LoxError> {
        for stmt in statements {
            self.execute(stmt)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: Stmt) -> Result<(), LoxError> {
        match stmt {
            Stmt::Print(expr) => {
                let value = self.evaluate(expr)?;
                self.print_value(value);
            }
            Stmt::Var(name, initializer) => {
                let value = self.evaluate(initializer)?;
                self.environment.define(name, value);
            }
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
            }
            Stmt::Block(statements) => self.execute_block(statements)?,
        }
        Ok(())
    }

    fn execute_block(&mut self, statements: Vec<Stmt>) -> Result<(), LoxError> {
        let previous_environment = self.environment.clone();
        self.environment = Environment::with_parent(previous_environment);

        for stmt in statements {
            self.execute(stmt)?;
        }

        self.environment = *self
            .environment
            .parent
            .clone()
            .unwrap_or_else(|| panic!("Parent environment was expected but not found."));

        Ok(())
    }

    pub fn evaluate(&mut self, expr: Expr) -> Result<Expr, LoxError> {
        match expr {
            Expr::String(s) => Ok(Expr::String(s)),
            Expr::Number(n) => Ok(Expr::Number(n)),
            Expr::Boolean(b) => Ok(Expr::Boolean(b)),
            Expr::Nil => Ok(Expr::Nil),
            Expr::Unary { operator, right } => {
                let right_val = self.evaluate(*right)?;
                let line = operator.line;
                match operator.token_type {
                    TokenType::Minus => {
                        if let Expr::Number(n) = right_val {
                            return Ok(Expr::Number(-n));
                        }
                        Err(LoxError::new("Operand must be a number", Some(line)))
                    }
                    TokenType::Bang => {
                        let is_truthy = self.is_truthy(&right_val);
                        Ok(Expr::Boolean(!is_truthy))
                    }
                    _ => Err(LoxError::new("Unknown unary operator", Some(line))),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate(*left)?;
                let right_val = self.evaluate(*right)?;
                let line = operator.line;
                self.handle_binary_op(left_val, &operator.token_type, right_val, line)
            }
            Expr::Grouping(inner_expr) => self.evaluate(*inner_expr),
            Expr::Assign { name, value } => {
                let evaluated_value = self.evaluate(*value)?;
                let key = name.clone();
                self.environment.assign(key, evaluated_value, 0)?;
                Ok(self.environment.get(&name).unwrap().clone())
            }
            Expr::Variable(name) => {
                if let Some(value) = self.environment.get(&name) {
                    Ok(value.clone())
                } else {
                    Err(LoxError::new("Undefined variable", None))
                }
            }
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
            Expr::Variable(name) => println!("{}", name),
            Expr::Assign { name, value } => println!("{} {}", name, value),
        }
    }

    fn handle_binary_op(
        &self,
        left: Expr,
        operator: &TokenType,
        right: Expr,
        line: usize,
    ) -> Result<Expr, LoxError> {
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
            _ => Ok(Expr::Nil),
        }
    }

    fn handle_plus(&self, left: Expr, right: Expr, line: usize) -> Result<Expr, LoxError> {
        match (left, right) {
            (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Number(l + r)),
            (Expr::String(l), Expr::String(r)) => Ok(Expr::String(format!("{}{}", l, r))),
            _ => Err(LoxError::new(
                "Operands must be two numbers or two strings",
                Some(line),
            )),
        }
    }

    fn handle_minus(&self, left: Expr, right: Expr, line: usize) -> Result<Expr, LoxError> {
        if let (Expr::Number(l), Expr::Number(r)) = (left, right) {
            Ok(Expr::Number(l - r))
        } else {
            Err(LoxError::new("Operands must be numbers.", Some(line)))
        }
    }

    fn handle_divide(&self, left: Expr, right: Expr, line: usize) -> Result<Expr, LoxError> {
        if let (Expr::Number(l), Expr::Number(r)) = (left, right) {
            if r == 0.0 {
                return Err(LoxError::new("Division by zero", Some(line)));
            }
            Ok(Expr::Number(l / r))
        } else {
            Err(LoxError::new("Operands must be numbers.", Some(line)))
        }
    }

    fn handle_multiply(&self, left: Expr, right: Expr, line: usize) -> Result<Expr, LoxError> {
        if let (Expr::Number(l), Expr::Number(r)) = (left, right) {
            Ok(Expr::Number(l * r))
        } else {
            Err(LoxError::new("Operands must be numbers.", Some(line)))
        }
    }

    fn handle_greater(&self, left: Expr, right: Expr, line: usize) -> Result<Expr, LoxError> {
        if let (Expr::Number(l), Expr::Number(r)) = (left, right) {
            Ok(Expr::Boolean(l > r))
        } else {
            Err(LoxError::new("Operands must be numbers.", Some(line)))
        }
    }

    fn handle_greater_equal(&self, left: Expr, right: Expr, line: usize) -> Result<Expr, LoxError> {
        if let (Expr::Number(l), Expr::Number(r)) = (left, right) {
            Ok(Expr::Boolean(l >= r))
        } else {
            Err(LoxError::new("Operands must be numbers.", Some(line)))
        }
    }

    fn handle_less(&self, left: Expr, right: Expr, line: usize) -> Result<Expr, LoxError> {
        if let (Expr::Number(l), Expr::Number(r)) = (left, right) {
            Ok(Expr::Boolean(l < r))
        } else {
            Err(LoxError::new("Operands must be numbers.", Some(line)))
        }
    }

    fn handle_less_equal(&self, left: Expr, right: Expr, line: usize) -> Result<Expr, LoxError> {
        if let (Expr::Number(l), Expr::Number(r)) = (left, right) {
            Ok(Expr::Boolean(l <= r))
        } else {
            Err(LoxError::new("Operands must be numbers.", Some(line)))
        }
    }

    fn handle_equal_equal(&self, left: Expr, right: Expr) -> Result<Expr, LoxError> {
        Ok(Expr::Boolean(left == right))
    }

    fn handle_bang_equal(&self, left: Expr, right: Expr) -> Result<Expr, LoxError> {
        Ok(Expr::Boolean(left != right))
    }

    fn is_truthy(&self, value: &Expr) -> bool {
        match value {
            Expr::Boolean(b) => *b,
            Expr::Nil => false,
            _ => true,
        }
    }
}
