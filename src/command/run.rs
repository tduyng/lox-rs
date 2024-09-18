use std::process;

use super::Command;
use crate::{
    error::{ExitCode, LoxError},
    interpreter::Interpreter,
    parser::Parser,
    scanner::Scanner,
};

pub struct RunCommand {
    file_contents: String,
}

impl RunCommand {
    pub fn new(file_contents: String) -> Self {
        Self { file_contents }
    }
}

impl Command for RunCommand {
    fn execute(&self) -> Result<ExitCode, LoxError> {
        let mut scanner = Scanner::new(self.file_contents.clone());
        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(tokens.to_vec(), true);
        if scanner.has_error() {
            process::exit(65);
        }

        match parser.parse() {
            Ok(statements) => {
                let mut interpreter = Interpreter::new();
                match interpreter.interpret(statements) {
                    Ok(_) => process::exit(0),
                    Err(e) => {
                        eprintln!("[line {}] Error: {}", e.line, e.message);
                        process::exit(70);
                    }
                }
            }
            Err(e) => {
                eprintln!("[line {}] Error: {}", e.line, e.message);
                process::exit(65);
            }
        }
    }
}
