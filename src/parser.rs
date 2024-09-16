use crate::{
    ast::Expr,
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
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.relational();

        while self.match_token(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator = self.previous().clone();
            let right = self.relational();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn relational(&mut self) -> Expr {
        let mut expr = self.addition();

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.addition(); // Changed from multiplication to addition
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
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

        let lexeme = token.lexeme.clone();
        let line = token.line;

        match token.token_type {
            TokenType::String => Expr::String(lexeme),
            TokenType::Number => Expr::Number(lexeme.parse().unwrap_or(0.0)),
            TokenType::True => Expr::Boolean(true),
            TokenType::False => Expr::Boolean(false),
            TokenType::Nil => Expr::Nil,
            TokenType::LeftParen => {
                let expr = self.expression();
                self.consume(TokenType::RightParen);
                Expr::Grouping(Box::new(expr))
            }
            _ => {
                self.report_error(&format!("Unexpected token: '{}'", lexeme), line);
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

    fn report_error(&self, message: &str, line: usize) {
        eprintln!("Error on line {}: {}", line, message);
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn consume(&mut self, expected: TokenType) {
        if self.is_at_end() || self.peek().token_type != expected {
            let current_token = self.peek();
            self.report_error(
                &format!(
                    "Expected {:?}, but got {:?} on line {}.",
                    expected, current_token.token_type, current_token.line
                ),
                current_token.line,
            );
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
