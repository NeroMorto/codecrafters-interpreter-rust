use std::fmt::{Display, Formatter};
use std::io;
use std::io::Write;
use std::iter::Peekable;
use std::slice::Iter;
use crate::token::{Token, TokenType};

enum Expression {
    Bool(bool),
    String(String),
    Number(f64),
    Nil,
    Grouping(Box<Expression>),
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
            Expression::String(literal) => write!(f, "{literal}"),
            Expression::Grouping(literal) => write!(f, "(group {literal})"),
        }
    }
}

pub struct Parser {
    expressions: Vec<Expression>,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            expressions: Vec::new(),
            errors: Vec::new()
        }
    }


    fn match_token(&mut self, tokens: &mut Peekable<Iter<Token>>)-> Option<Expression> {
        match &tokens.next().unwrap().token_type {
            TokenType::False => Some(Expression::Bool(false)),
            TokenType::True => Some(Expression::Bool(true)),
            TokenType::Nil => Some(Expression::Nil),
            TokenType::Number(literal) => Some(Expression::Number(*literal)),
            TokenType::String(literal) => Some(Expression::String(literal.clone())),
            _ => None
        }
    }

    fn match_group(&mut self, tokens: &mut Peekable<Iter<Token>>) -> Result<Expression, String> {
        tokens.next(); // Consuming left paren
        let mut expression: Option<Expression> = None;

        while let Some(token) = tokens.peek() {
            match token.token_type {
                TokenType::LeftParen => {
                    expression = Some(self.match_group(tokens)?);
                    // expression = Some(Expression::Grouping(Box::new(group_expr)));
                },
                TokenType::RightParen => {
                    tokens.next(); // Consuming right paren
                    match expression {
                        None => return Err("Error: Empty group".into()),
                        Some(exp) => return Ok(Expression::Grouping(Box::new(exp)))
                    }
                },
                _ => {
                    expression = self.match_token(tokens);
                }
            }
        }

        Err("Error: Unmatched parentheses.".into())
    }

    pub fn parse_peekable(&mut self, tokens: &Vec<Token>) {
        let mut tokens_peekable = tokens.iter().peekable();
        while let Some(token) = tokens_peekable.peek() {
            match token.token_type {

                TokenType::LeftParen => {
                    match self.match_group(&mut tokens_peekable) {
                        Ok(group) => self.expressions.push(group),
                        Err(err) => self.errors.push(err)
                    }
                }

                _ => {

                    match self.match_token(&mut tokens_peekable) {
                        None => {}
                        Some(expression) => self.expressions.push(expression),
                    }
                    // self.expressions.push()
                }
            }

        }
    }

    fn parse_group(&mut self, tokens: &Vec<Token>) {}

    pub fn print_expressions<T: Write>(&self, mut buffer: T) -> Result<(), io::Error> {
        for expression in &self.expressions {
            writeln!(buffer, "{expression}")?
        }
        Ok(())
    }
}
