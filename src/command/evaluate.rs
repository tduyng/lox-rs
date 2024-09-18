use std::process;

use crate::{
    ast::Stmt, error::ExitCode, interpreter::Interpreter, parser::Parser, scanner::Scanner,
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
        let statement = match parser.parse() {
            Ok(stmt) => stmt,
            Err(e) => {
                eprintln!("Parsing error: {}", e);
                process::exit(65);
            }
        };
        let mut interpreter = Interpreter::new();
        for statement in statement {
            match statement {
                Stmt::Expression(expr) => {
                    let expr = interpreter.evaluate(expr);
                    interpreter.print_value(expr);
                }
                _ => {}
            }
        }

        process::exit(0)
    }
}
