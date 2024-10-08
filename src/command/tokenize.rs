use std::process;

use crate::{
    error::{ExitCode, LoxError},
    scanner::Scanner,
    token::TokenType,
    utils::pad_number,
};

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
    fn execute(&self) -> Result<ExitCode, LoxError> {
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
                    pad_number(value.parse::<f64>().unwrap_or(0.0))
                }
                Some(value) => value.clone(),
                None => "null".to_string(),
            };

            println!("{} {} {}", token_type, lexeme, literal_str);
        }

        if scanner.has_error() {
            process::exit(65);
        }

        process::exit(0)
    }
}
