use std::process;

use crate::{
    ast::Stmt,
    error::{ExitCode, LoxError},
    interpreter::Interpreter,
    parser::Parser,
    scanner::Scanner,
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
    fn execute(&self) -> Result<ExitCode, LoxError> {
        let mut scanner = Scanner::new(self.file_contents.clone());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens.to_vec(), false);

        if scanner.has_error() {
            process::exit(65);
        }
        let statement = match parser.parse() {
            Ok(stmt) => stmt,
            Err(e) => {
                eprintln!("[line {}] Error: {}", e.line, e.message);
                process::exit(65);
            }
        };
        let mut interpreter = Interpreter::new();
        for statement in statement {
            if let Stmt::Expression(expr) = statement {
                let expr = match interpreter.evaluate(expr) {
                    Ok(value) => value,
                    Err(e) => {
                        eprintln!("[line {}] Error: {}", e.line, e.message);
                        process::exit(70);
                    }
                };
                interpreter.print_value(expr);
            }
        }

        process::exit(0)
    }
}
