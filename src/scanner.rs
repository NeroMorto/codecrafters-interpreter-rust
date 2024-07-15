use std::io;
use std::iter::Peekable;
use std::str::Chars;
use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    content: &'a String,
    current_line_number: usize,
    pub tokens: Vec<Token>,
    errors: Vec<String>,
}

impl<'a> Scanner<'a> {
    pub fn new(content: &'a String) -> Self {
        Scanner {
            content,
            current_line_number: 0,
            tokens: vec![],
            errors: vec![],
        }
    }
    pub fn scan(&mut self) {
        for (line_number, line) in self.content.lines().enumerate() {
            self.scan_line(line_number, line)
        }
        self.emit_token(TokenType::Eof,  ' ')
    }

    fn scan_line(&mut self, line_number: usize, line: &'a str) {
        self.current_line_number = line_number + 1;
        let mut chars = line.chars().peekable();
        while let Some(char) = chars.next() {
            self.match_char(char, &mut chars)
        }
    }

    fn emit_token<T: Into<String>>(&mut self, token_type: TokenType, lexeme: T) {
        let lexeme = lexeme.into();
        self.tokens.push(Token { token_type, lexeme })
    }

    fn match_double_char(&mut self, char: char, chars: &mut Peekable<Chars>) {
        let token_type = match char {
            '=' => (TokenType::EqualEqual, TokenType::Equal),
            '<' => (TokenType::LessEqual, TokenType::Less),
            '>' => (TokenType::GreaterEqual, TokenType::Greater),
            '!' => (TokenType::BangEqual, TokenType::Bang),
            _ => return, // This should not be reached
        };

        if let Some('=') = chars.peek() {
            self.emit_token(token_type.0, format!("{}{}", char, chars.next().unwrap()));
        } else {
            self.emit_token(token_type.1, char);
        }
    }
    fn match_char(&mut self, char: char, chars: &mut Peekable<Chars>) {
        match char {
            ' ' | '\t' => {}
            '(' => self.emit_token(TokenType::LeftParen, char),
            ')' => self.emit_token(TokenType::RightParen, char),
            '{' => self.emit_token(TokenType::LeftBrace, char),
            '}' => self.emit_token(TokenType::RightBrace, char),
            '*' => self.emit_token(TokenType::Star, char),
            '-' => self.emit_token(TokenType::Minus, char),
            '+' => self.emit_token(TokenType::Plus, char),
            '.' => self.emit_token(TokenType::Dot, char),
            ',' => self.emit_token(TokenType::Comma, char),
            ';' => self.emit_token(TokenType::Semicolon, char),
            '=' | '<' | '>' | '!' => self.match_double_char(char, chars),
            '0'..='9' => {
                let mut digits_with_possible_dot = format!("{}", char);
                // TODO Have to find a less dirty workaround
                let mut is_dot_found = false;
                let mut is_dot_fits_in_digit = false;
                loop {
                    match chars.peek() {
                        Some('0'..='9') => digits_with_possible_dot.push(chars.next().unwrap()),
                        Some('.') => {
                            is_dot_found = true;
                            if is_dot_fits_in_digit {
                                break;
                            }
                            let dot = chars.next().unwrap();
                            if let Some('0'..='9') = chars.peek() {
                                digits_with_possible_dot.push(dot);
                                is_dot_fits_in_digit = true;
                            }else {
                                break
                            }
                        }
                        _ => break,
                    }
                }


                self.emit_token(
                    TokenType::Number(digits_with_possible_dot.parse::<f64>().unwrap()),
                    digits_with_possible_dot,
                );
                if is_dot_found && !is_dot_fits_in_digit {
                    self.emit_token(TokenType::Dot, '.')
                }
            }
            '/' => {
                if let Some(&'/') = chars.peek() {
                    // Consuming rest of the line, probably mast be a way to go to the next line
                    for _ in chars.by_ref() {}
                } else {
                    self.emit_token(TokenType::Slash, char);
                }
            }
            '"' => {
                let mut literal = "".to_string();
                let mut is_complete = false;

                for next_char in chars.by_ref() {
                    if next_char == '"' {
                        is_complete = true;
                        break;
                    }
                    literal.push(next_char)
                }

                if is_complete {
                    self.emit_token(TokenType::String(literal.clone()), literal);
                } else {
                    self.add_error(format!(
                        "[line {}] Error: Unterminated string.",
                        self.current_line_number
                    ))
                }
            }
            another_char => {
                if another_char.is_ascii_alphabetic()
                    || another_char.is_ascii_digit()
                    || another_char == '_'
                {
                    let mut identifier = format!("{another_char}");
                    while let Some(next_char) = chars.peek() {
                        if !(next_char.is_ascii_alphabetic()
                            || next_char.is_ascii_digit()
                            || *next_char == '_')
                        {
                            break;
                        }
                        identifier.push(chars.next().unwrap());
                    }

                    // let token = match_reserved(&identifier);
                    self.emit_token(Scanner::match_reserved(&identifier), identifier);
                } else {
                    self.add_error(format!(
                        "[line {}] Error: Unexpected character: {}",
                        self.current_line_number, another_char
                    ))
                }
            }
        }
    }

    fn add_error(&mut self, error: String) {
        self.errors.push(error)
    }

    fn match_reserved(identifier: &str) -> TokenType {
        match identifier {
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
        }
    }

    pub fn write_tokens<T: io::Write>(&self, mut buffer: T) -> Result<(), io::Error> {
        for token in &self.tokens {
            writeln!(buffer, "{token}")?
        }
        Ok(())
    }

    pub fn write_errors<T: io::Write>(&self, mut buffer: T) -> Result<(), io::Error> {
        for error in &self.errors {
            writeln!(buffer, "{error}")?
        }
        Ok(())
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}