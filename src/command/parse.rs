use super::Command;
use crate::{
    ast::Expr, error::ExitCode, parser::Parser, scanner::Scanner, utils::format_parsed_number,
};
use std::process;

pub struct ParseCommand {
    file_contents: String,
}

impl ParseCommand {
    pub fn new(file_contents: String) -> Self {
        Self { file_contents }
    }
}

impl Command for ParseCommand {
    fn execute(&self) -> ExitCode {
        let mut scanner = Scanner::new(self.file_contents.clone());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens.to_vec());
        let expression = parser.parse();

        if parser.had_errors() {
            eprintln!("Parsing errors encountered.");
            process::exit(65);
        }

        match &expression {
            Expr::String(s) => println!("{}", s),
            Expr::Number(n) => {
                let formatted_number = format_parsed_number(*n);
                println!("{}", formatted_number);
            }
            Expr::Boolean(b) => println!("{}", b),
            Expr::Nil => println!("nil"),
            Expr::Unary { operator, right } => {
                let right = match **right {
                    Expr::Number(n) => format_parsed_number(n),
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
                    Expr::Number(n) => format_parsed_number(n),
                    _ => left.to_string(),
                };
                let right = match **right {
                    Expr::Number(n) => format_parsed_number(n),
                    _ => right.to_string(),
                };
                println!("({} {} {})", operator.lexeme, left, right);
            }
            Expr::Grouping(expr) => {
                let expr = match **expr {
                    Expr::Number(n) => format_parsed_number(n),
                    _ => expr.to_string(),
                };
                println!("(group {})", expr)
            }
        }

        process::exit(0)
    }
}
