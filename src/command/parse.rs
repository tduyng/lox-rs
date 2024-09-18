use super::Command;
use crate::{
    ast::Stmt,
    error::{ExitCode, LoxError},
    parser::Parser,
    scanner::Scanner,
};
use std::process;

pub struct ParseCommand {
    file_contents: String,
}

impl ParseCommand {
    pub fn new(file_contents: String) -> Self {
        Self { file_contents }
    }

    fn handle_statement(&self, stmt: Stmt) {
        if let Stmt::Expression(expr) = stmt {
            println!("{}", expr);
        }
    }
}

impl Command for ParseCommand {
    fn execute(&self) -> Result<ExitCode, LoxError> {
        let mut scanner = Scanner::new(self.file_contents.clone());
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens.to_vec(), false);

        if scanner.has_error() {
            process::exit(65);
        }

        match parser.parse() {
            Ok(statement) => {
                for stmt in statement {
                    self.handle_statement(stmt);
                }
                process::exit(0)
            }
            Err(e) => {
                if let Some(line) = e.line {
                    eprintln!("[line {}] Error: {}", line, e.message);
                } else {
                    eprintln!("Error: {}", e.message);
                }
                process::exit(65)
            }
        }
    }
}
