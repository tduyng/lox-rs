use std::process;

use crate::{error::ExitCode, scanner::Scanner, token::TokenType, utils::format_tokenized_number};

use super::Command;

pub struct TokenizeCommand {
    file_contents: String,
}

impl TokenizeCommand {
    pub fn new(file_contents: String) -> Self {
        Self { file_contents }
    }
}

impl Command for TokenizeCommand {
    fn execute(&self) -> ExitCode {
        let mut scanner = Scanner::new(self.file_contents.clone());
        let tokens = scanner.scan_tokens();

        for token in tokens {
            let token_type = token.token_type.to_string();
            let lexeme = if token.token_type == TokenType::String {
                format!("\"{}\"", token.lexeme)
            } else {
                token.lexeme.clone()
            };

            let literal_str = match &token.literal {
                Some(value) if token.token_type == TokenType::Number => {
                    format_tokenized_number(value)
                }
                Some(value) => value.clone(),
                None => "null".to_string(),
            };

            println!("{} {} {}", token_type, lexeme, literal_str);
        }
        if scanner.had_errors() {
            process::exit(65);
        }
        process::exit(0)
    }
}
