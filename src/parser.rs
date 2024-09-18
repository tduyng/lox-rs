use crate::{
    ast::{Expr, Stmt},
    error::LoxError,
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    require_semicolon: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, require_semicolon: bool) -> Self {
        Self {
            tokens,
            current: 0,
            require_semicolon,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.statement()?);
        }

        Ok(statements)
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.match_token(&[TokenType::Print]) {
            return self.print_statement();
        }

        if self.match_token(&[TokenType::Var]) {
            return self.var_declaration();
        }

        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon)?;
        Ok(Stmt::Print(value))
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = if let TokenType::Identifier = self.peek().token_type {
            self.peek().lexeme.clone()
        } else {
            return Err(LoxError::new(
                "Expected variable name after 'var'",
                Some(self.peek().line),
            ));
        };

        self.advance();

        let initializer = if self.match_token(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon)?;

        Ok(Stmt::Var(name, initializer.unwrap_or(Expr::Nil)))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        if self.require_semicolon {
            self.consume(TokenType::Semicolon)?;
        }
        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.equality()?;

        if self.match_token(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expr::Variable(ref name) = expr {
                return Ok(Expr::Assign {
                    name: name.clone(),
                    value: Box::new(value),
                });
            } else {
                return Err(LoxError::new(
                    "Invalid assignment target",
                    Some(equals.line),
                ));
            }
        }

        Ok(expr)
    }
    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.relational()?;

        while self.match_token(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator = self.previous().clone();
            let right = self.relational()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn relational(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.addition()?;

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.addition()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn addition(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.multiplication()?;

        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.multiplication()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn multiplication(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = Box::new(self.unary()?);
            return Ok(Expr::Unary { operator, right });
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.is_at_end() {
            return Err(LoxError::new(
                "Unexpected end of input",
                Some(self.peek().line),
            ));
        }

        let token = self.advance();
        let lexeme = token.lexeme.clone();
        match token.token_type {
            TokenType::String => Ok(Expr::String(lexeme)),
            TokenType::Number => Ok(Expr::Number(lexeme.parse::<f64>().unwrap_or(0.0))),
            TokenType::True => Ok(Expr::Boolean(true)),
            TokenType::False => Ok(Expr::Boolean(false)),
            TokenType::Nil => Ok(Expr::Nil),
            TokenType::Identifier => Ok(Expr::Variable(lexeme)),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen)?;
                Ok(Expr::Grouping(Box::new(expr)))
            }
            _ => Err(LoxError::new(
                &format!("Unexpected token: '{}'", token.lexeme),
                Some(token.line),
            )),
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        // self.current >= self.tokens.len()
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn consume(&mut self, expected: TokenType) -> Result<(), LoxError> {
        if self.is_at_end() {
            let eof_token = self.peek();
            return Err(LoxError::new(
                &format!("Expected {:?}, but got Eof", expected),
                Some(eof_token.line),
            ));
        } else if self.peek().token_type != expected {
            let current_token = self.peek();
            return Err(LoxError::new(
                &format!(
                    "Expected {:?}, but got {:?}",
                    expected, current_token.token_type
                ),
                Some(current_token.line),
            ));
        }
        self.advance();
        Ok(())
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
