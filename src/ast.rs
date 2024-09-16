use std::fmt;

use crate::{token::Token, utils::format_evaluated_number};

#[derive(PartialEq)]
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
}

impl fmt::Display for Expr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::String(s) => write!(fmt, "{}", s),
            Expr::Number(n) => write!(fmt, "{}", format_evaluated_number(*n)),
            Expr::Boolean(b) => write!(fmt, "{}", b),
            Expr::Nil => write!(fmt, "nil"),
            Expr::Unary { operator, right } => write!(fmt, "({} {})", operator.lexeme, right),
            Expr::Binary {
                left,
                operator,
                right,
            } => write!(fmt, "({} {} {})", operator.lexeme, left, right),
            Expr::Grouping(expr) => match **expr {
                Expr::Number(n) => {
                    let formatted_number = format_evaluated_number(n);
                    write!(fmt, "(group {})", formatted_number)
                }
                _ => write!(fmt, "(group {})", expr),
            },
        }
    }
}
