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
    errors: Vec<LoxError>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            errors: Vec::new(),
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
            'p' if self.look_ahead(5) == "print" => {
                self.add_token(TokenType::Print);
                self.current += 4;
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
                    if self.peek() == '\n' {
                        self.line += 1;
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '"' => self.string(),
            '=' => {
                if self.match_next('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '!' => {
                if self.match_next('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            ';' => self.add_token(TokenType::Semicolon),
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            ' ' | '\r' | '\t' => {
                // Ignore other whitespace characters
            }
            '\n' => self.line += 1,
            c if c.is_alphabetic()
                || c == '_'
                || c.is_ascii_punctuation()
                || c.is_ascii_whitespace() =>
            {
                self.identifier()
            }
            c if c.is_ascii_digit() => self.number(),
            _ => self.report_error(LoxError::new(
                &format!("Unexpected character: {}", c),
                self.line,
            )),
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current..].chars().next().unwrap();
        self.current += c.len_utf8();
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, lexeme, None, self.line));
    }

    fn string(&mut self) {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.report_error(LoxError::new("Unterminated string literal", self.line));
            return;
        }

        self.advance(); // Skip the closing quote

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.tokens.push(Token::new(
            TokenType::String,
            value.clone(),
            Some(value),
            self.line,
        ));
    }

    fn number(&mut self) {
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
    }

    fn identifier(&mut self) {
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

    fn report_error(&mut self, error: LoxError) {
        eprintln!("{}", error);
        self.errors.push(error);
    }

    pub fn had_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}
