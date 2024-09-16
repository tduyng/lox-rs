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
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.addition()
    }

    fn addition(&mut self) -> Expr {
        let mut expr = self.multiplication();

        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.multiplication();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn multiplication(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_token(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = Box::new(self.unary());
            return Expr::Unary { operator, right };
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.is_at_end() {
            return Expr::Nil;
        }

        let token = self.advance();
        match token.token_type {
            TokenType::String => Expr::String(token.lexeme.clone()),
            TokenType::Number => Expr::Number(token.lexeme.parse().unwrap_or(0.0)),
            TokenType::True => Expr::Boolean(true),
            TokenType::False => Expr::Boolean(false),
            TokenType::Nil => Expr::Nil,
            TokenType::LeftParen => {
                let expr = self.expression();
                self.consume(TokenType::RightParen);
                expr
            }
            _ => {
                self.report_error(ScannerError::UnexpectedCharacter(' ', 1));
                Expr::Nil
            }
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

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn consume(&mut self, expected: TokenType) {
        if self.is_at_end() || self.peek().token_type != expected {
            self.report_error(ScannerError::UnexpectedCharacter(' ', self.current));
        } else {
            self.advance();
        }
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == *token_type
        }
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}

pub enum Expr {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
}
