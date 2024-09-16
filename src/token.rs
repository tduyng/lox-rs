use std::fmt;

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub enum TokenType {
    // Single-character tokens
    RightParen,
    RightBrace,
    LeftParen,
    LeftBrace,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    Semicolon,
    Slash,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Literals
    String,
    Number,
    Identifier,

    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<String>,
        line: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::RightBrace => write!(fmt, "RIGHT_BRACE"),
            TokenType::LeftBrace => write!(fmt, "LEFT_BRACE"),
            TokenType::RightParen => write!(fmt, "RIGHT_PAREN"),
            TokenType::LeftParen => write!(fmt, "LEFT_PAREN"),
            TokenType::Star => write!(fmt, "STAR"),
            TokenType::Dot => write!(fmt, "DOT"),
            TokenType::Comma => write!(fmt, "COMMA"),
            TokenType::Plus => write!(fmt, "PLUS"),
            TokenType::Minus => write!(fmt, "MINUS"),
            TokenType::Semicolon => write!(fmt, "SEMICOLON"),
            TokenType::Slash => write!(fmt, "SLASH"),
            TokenType::Bang => write!(fmt, "BANG"),
            TokenType::BangEqual => write!(fmt, "BANG_EQUAL"),
            TokenType::Equal => write!(fmt, "EQUAL"),
            TokenType::EqualEqual => write!(fmt, "EQUAL_EQUAL"),
            TokenType::Less => write!(fmt, "LESS"),
            TokenType::LessEqual => write!(fmt, "LESS_EQUAL"),
            TokenType::Greater => write!(fmt, "GREATER"),
            TokenType::GreaterEqual => write!(fmt, "GREATER_EQUAL"),
            TokenType::String => write!(fmt, "STRING"),
            TokenType::Number => write!(fmt, "NUMBER"),
            TokenType::Identifier => write!(fmt, "IDENTIFIER"),
            TokenType::And => write!(fmt, "AND"),
            TokenType::Class => write!(fmt, "CLASS"),
            TokenType::Else => write!(fmt, "ELSE"),
            TokenType::False => write!(fmt, "FALSE"),
            TokenType::For => write!(fmt, "FOR"),
            TokenType::Fun => write!(fmt, "FUN"),
            TokenType::If => write!(fmt, "IF"),
            TokenType::Nil => write!(fmt, "NIL"),
            TokenType::Or => write!(fmt, "OR"),
            TokenType::Print => write!(fmt, "PRINT"),
            TokenType::Return => write!(fmt, "RETURN"),
            TokenType::Super => write!(fmt, "SUPER"),
            TokenType::This => write!(fmt, "THIS"),
            TokenType::True => write!(fmt, "TRUE"),
            TokenType::Var => write!(fmt, "VAR"),
            TokenType::While => write!(fmt, "WHILE"),
            TokenType::Eof => write!(fmt, "EOF"),
        }
    }
}
