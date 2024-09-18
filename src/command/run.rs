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

        let mut parser = Parser::new(tokens.to_vec());
        if scanner.has_error() {
            process::exit(65);
        }

        match parser.parse() {
            Ok(statements) => {
                let mut interpreter = Interpreter::new();
                interpreter.interpret(statements)?;
                process::exit(0);
            }
            Err(e) => {
                eprintln!("{}", e);
                process::exit(65);
            }
        }
    }
}
