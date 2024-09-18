use crate::{
    error::LoxError,
    token::{Token, TokenType},
};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    has_error: bool,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            has_error: false,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            if let Err(err) = self.scan_token() {
                self.report_error(err);
                self.has_error = true;
            }
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));

        &self.tokens
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();
        match c {
            'p' if self.look_ahead(5) == "print" => {
                self.add_token(TokenType::Print)?;
                self.current += 4;
                Ok(())
            }
            'v' if self.look_ahead(2) == "var" => {
                self.add_token(TokenType::Var)?;
                self.current += 2;
                Ok(())
            }
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Star),
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(())
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            '"' => self.string(),
            '=' => {
                if self.match_next('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '!' => {
                if self.match_next('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            ';' => self.add_token(TokenType::Semicolon),
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            ' ' | '\r' | '\t' => Ok(()), // Ignore whitespace
            '\n' => {
                self.line += 1;
                Ok(())
            }
            c if c.is_alphabetic() || c == '_' => self.identifier(),
            c if c.is_ascii_digit() => self.number(),
            _ => Err(LoxError::new(
                &format!("Unexpected character: {}", c),
                Some(self.line),
            )),
        }
    }

    fn add_token(&mut self, token_type: TokenType) -> Result<(), LoxError> {
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, lexeme, None, self.line));
        Ok(())
    }

    fn string(&mut self) -> Result<(), LoxError> {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LoxError::new("Unterminated string.", Some(self.line)));
        }

        self.advance(); // Skip the closing quote

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.tokens.push(Token::new(
            TokenType::String,
            value.clone(),
            Some(value),
            self.line,
        ));
        Ok(())
    }

    fn number(&mut self) -> Result<(), LoxError> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value = self.source[self.start..self.current].to_string();

        self.tokens.push(Token::new(
            TokenType::Number,
            value.clone(),
            Some(value),
            self.line,
        ));
        Ok(())
    }

    fn identifier(&mut self) -> Result<(), LoxError> {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let lexeme = self.source[self.start..self.current].to_string();
        let token_type = match lexeme.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };
        self.tokens
            .push(Token::new(token_type, lexeme, None, self.line));
        Ok(())
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current..].chars().next().unwrap();
        self.current += c.len_utf8();
        c
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        self.source[self.current..].chars().next().unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current..].chars().nth(1).unwrap_or('\0')
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek() == '\0' || self.current >= self.source.len()
    }

    fn look_ahead(&self, length: usize) -> String {
        let start = self.current;
        let end = (start + length).min(self.source.len());
        self.source[start..end].to_string()
    }

    fn report_error(&self, error: LoxError) {
        if let Some(line) = error.line {
            eprintln!("[line {}] Error: {}", line, error.message);
        } else {
            eprintln!("Error: {}", error.message);
        }
    }

    pub fn has_error(&self) -> bool {
        self.has_error
    }
}
