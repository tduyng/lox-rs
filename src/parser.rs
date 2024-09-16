use crate::{
    error::ScannerError,
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<ScannerError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Expr {
        if !self.is_at_end() {
            let token = self.advance();
            match token.token_type {
                TokenType::Number => Expr::Number(token.lexeme.parse().unwrap_or(0.0)),
                TokenType::True => Expr::Boolean(true),
                TokenType::False => Expr::Boolean(false),
                TokenType::Nil => Expr::Nil,
                _ => {
                    self.report_error(ScannerError::UnexpectedCharacter(' ', 1));
                    Expr::Nil
                }
            }
        } else {
            Expr::Nil
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    pub fn had_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn report_error(&mut self, error: ScannerError) {
        eprintln!("{}", error);
        self.errors.push(error);
    }
}

pub enum Expr {
    Number(f64),
    Boolean(bool),
    Nil,
}
