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
        let expression = parser.parse();

        if parser.had_errors() {
            eprintln!("Parsing errors encountered.");
            process::exit(65);
        }

        println!("{}", expression);

        process::exit(0)
    }
}
