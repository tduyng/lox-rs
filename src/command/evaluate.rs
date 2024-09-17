use std::process;

use crate::{
    ast::Expr, error::ExitCode, interpreter::Interpreter, parser::Parser, scanner::Scanner,
    utils::format_evaluated_number,
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
        let expression = parser.parse();
        if parser.had_errors() {
            eprintln!("Parsing errors encountered.");
            process::exit(65);
        }
        let mut interpreter = Interpreter::new();
        let result = interpreter.evaluate(expression);

        match &result {
            Expr::String(s) => println!("{}", s),
            Expr::Number(n) => println!("{}", format_evaluated_number(*n)),
            Expr::Boolean(b) => println!("{}", b),
            Expr::Nil => println!("nil"),
            Expr::Unary { operator, right } => {
                let right = match **right {
                    Expr::Number(n) => format_evaluated_number(n),
                    _ => right.to_string(),
                };
                println!("({} {})", operator.lexeme, right)
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = match **left {
                    Expr::Number(n) => format_evaluated_number(n),
                    _ => left.to_string(),
                };
                let right = match **right {
                    Expr::Number(n) => format_evaluated_number(n),
                    _ => right.to_string(),
                };
                println!("({} {} {})", operator.lexeme, left, right)
            }
            Expr::Grouping(expr) => {
                let expr = match **expr {
                    Expr::Number(n) => format_evaluated_number(n),
                    _ => expr.to_string(),
                };
                println!("(group {})", expr)
            }
        }

        process::exit(0)
    }
}
