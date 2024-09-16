use std::process;

use crate::{error::ExitCode, interpreter::Interpreter, parser::Parser, scanner::Scanner};

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
        println!("{}", result);

        process::exit(0)
    }
}
