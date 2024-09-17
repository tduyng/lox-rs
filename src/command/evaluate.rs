use std::process;

use crate::{
    ast::Expr, error::ExitCode, interpreter::Interpreter, parser::Parser, scanner::Scanner,
};

use super::Command;

pub struct EvaluateCommand {
    file_contents: String,
}

impl EvaluateCommand {
    pub fn new(file_contents: String) -> Self {
        Self { file_contents }
    }
}

impl Command for EvaluateCommand {
    fn execute(&self) -> ExitCode {
        let mut scanner = Scanner::new(self.file_contents.clone());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens.to_vec());
        let expression = match parser.parse() {
            Ok(expr) => expr,
            Err(e) => {
                eprintln!("Parsing error: {}", e);
                process::exit(65);
            }
        };

        if parser.had_errors() {
            eprintln!("Parsing errors encountered.");
            process::exit(65);
        }
        let mut interpreter = Interpreter::new();
        let result = interpreter.evaluate(expression);

        match result {
            Expr::String(s) => println!("{}", s),
            Expr::Number(n) => println!("{}", n),
            Expr::Boolean(b) => println!("{}", b),
            Expr::Nil => println!("nil"),
            Expr::Unary { operator, right } => println!("({} {})", operator.lexeme, right),
            Expr::Binary {
                left,
                operator,
                right,
            } => println!("({} {} {})", operator.lexeme, left, right),
            Expr::Grouping(expr) => println!("(group {})", expr),
        }

        process::exit(0)
    }
}
