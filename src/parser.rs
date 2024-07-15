use std::fmt::{Display, Formatter};
use std::io;
use std::io::Write;

use crate::token::{Token, TokenType};

enum Expression {
    Bool(bool),
    String(String),
    Number(f64),
    Nil,
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Bool(literal) => write!(f, "{literal}"),
            Expression::Nil => write!(f, "nil"),
            Expression::Number(literal) => {
                if literal.fract() == 0.0 {
                    write!(f, "{:.1}", literal)
                } else {
                    write!(f, "{}", literal)
                }
            }
            Expression::String(literal) => write!(f, "{literal}")
        }
    }
}

pub struct Parser {
    expressions: Vec<Expression>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            expressions: vec![],
        }
    }

    pub fn parse(&mut self, tokens: &Vec<Token>) {
        for token in tokens {
            match &token.token_type {
                TokenType::False => self.expressions.push(Expression::Bool(false)),
                TokenType::True => self.expressions.push(Expression::Bool(true)),
                TokenType::Nil => self.expressions.push(Expression::Nil),
                TokenType::Number(literal) => self.expressions.push(Expression::Number(*literal)),
                TokenType::String(literal) => self.expressions.push(Expression::String(literal.clone())),
                _ => {}
            }
        }
    }

    pub fn print_expressions<T: Write>(&self, mut buffer: T) -> Result<(), io::Error> {
        for expression in &self.expressions {
            writeln!(buffer, "{expression}")?
        }
        Ok(())
    }
}
