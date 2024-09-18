use std::fmt;

use crate::{token::Token, utils::pad_number};

#[derive(PartialEq, Clone, Debug)]
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
    Grouping(Box<Expr>),
    Variable {
        operator: Token,
        name: String,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::String(s) => write!(fmt, "{}", s),
            Expr::Number(n) => write!(fmt, "{}", pad_number(*n)),
            Expr::Boolean(b) => write!(fmt, "{}", b),
            Expr::Nil => write!(fmt, "nil"),
            Expr::Unary { operator, right } => write!(fmt, "({} {})", operator.lexeme, right),
            Expr::Binary {
                left,
                operator,
                right,
            } => write!(fmt, "({} {} {})", operator.lexeme, left, right),
            Expr::Grouping(expr) => write!(fmt, "(group {})", expr),
            Expr::Variable { operator: _, name } => write!(fmt, "{}", name),
        }
    }
}

pub enum Stmt {
    Print(Expr),
    Expression(Expr),
    Var(String, Expr),
}
