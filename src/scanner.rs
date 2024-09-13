use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '=' => self.add_token(TokenType::Equal),
            '}' => self.add_token(TokenType::RightBrace),
            '{' => self.add_token(TokenType::LeftBrace),
            '*' => self.add_token(TokenType::Star),
            '.' => self.add_token(TokenType::Dot),
            ',' => self.add_token(TokenType::Comma),
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            ';' => self.add_token(TokenType::Semicolon),
            '"' => self.string(),
            'a'..='z' | 'A'..='Z' => self.identifier(),
            _ => {
                // Placeholder for other cases
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, lexeme, None, self.line));
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            println!("Error: Unterminated string");
            return;
        }

        self.advance();
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.tokens.push(Token::new(
            TokenType::String,
            value.clone(),
            Some(value),
            self.line,
        ));
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = match text {
            "var" => TokenType::Var,
            "and" => TokenType::And,
            "class" => TokenType::Class,
            _ => TokenType::Identifier,
        };

        self.add_token(token_type);
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
