use super::Command;
use crate::{error::ExitCode, parser::Parser, scanner::Scanner};
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

        match parser.parse() {
            Ok(expression) => {
                println!("{}", expression);
                process::exit(0)
            }
            Err(e) => {
                eprintln!("{}", e);
                process::exit(65)
            }
        }
    }
}
