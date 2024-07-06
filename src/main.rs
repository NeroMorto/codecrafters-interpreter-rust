use std::env;
use std::fmt::{Display, Formatter};
use std::fs;
use std::io::{self, Write};
use std::iter::Peekable;
use std::process::exit;
use std::str::Chars;

enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Slash,
    DoubleQuote,
    String,
    Number,
    Identifier,
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
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "LEFT_PAREN"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN"),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE"),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE"),
            TokenType::Comma => write!(f, "COMMA"),
            TokenType::Dot => write!(f, "DOT"),
            TokenType::Minus => write!(f, "MINUS"),
            TokenType::Plus => write!(f, "PLUS"),
            TokenType::Semicolon => write!(f, "SEMICOLON"),
            TokenType::Star => write!(f, "STAR"),
            TokenType::Equal => write!(f, "EQUAL"),
            TokenType::EqualEqual => write!(f, "EQUAL_EQUAL"),
            TokenType::Bang => write!(f, "BANG"),
            TokenType::BangEqual => write!(f, "BANG_EQUAL"),
            TokenType::Less => write!(f, "LESS"),
            TokenType::LessEqual => write!(f, "LESS_EQUAL"),
            TokenType::Greater => write!(f, "GREATER"),
            TokenType::GreaterEqual => write!(f, "GREATER_EQUAL"),
            TokenType::Slash => write!(f, "SLASH"),
            TokenType::DoubleQuote => write!(f, "\""),
            TokenType::String => write!(f, "STRING"),
            TokenType::Number => write!(f, "NUMBER"),
            TokenType::Identifier => write!(f, "IDENTIFIER"),
            TokenType::And => write!(f, "AND"),
            TokenType::Class => write!(f, "CLASS"),
            TokenType::Else => write!(f, "ELSE"),
            TokenType::False => write!(f, "FALSE"),
            TokenType::For => write!(f, "FOR"),
            TokenType::Fun => write!(f, "FUN"),
            TokenType::If => write!(f, "IF"),
            TokenType::Nil => write!(f, "NIL"),
            TokenType::Or => write!(f, "OR"),
            TokenType::Print => write!(f, "PRINT"),
            TokenType::Return => write!(f, "RETURN"),
            TokenType::Super => write!(f, "SUPER"),
            TokenType::This => write!(f, "THIS"),
            TokenType::True => write!(f, "TRUE"),
            TokenType::Var => write!(f, "VAR"),
            TokenType::While => write!(f, "WHILE"),
        }
    }
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
        _ => TokenType::Identifier
    }
}

fn match_token(char: &char, chars: &mut Peekable<Chars>, line_number: &usize, is_error: &mut bool) -> Result<(), ()> {
    match char {
        'a'..='z' | 'A'..='Z' | '_' => {
            let mut identifier = format!("{char}");
            while let Some(next_char) = chars.peek() {
                if !('a'..='z').contains(next_char) && !('A'..='Z').contains(next_char) && !('0'..='9').contains(next_char) && '_' != *next_char {
                    break;
                }
                identifier.push(chars.next().unwrap());
            }

            let token =  match_reserved(&identifier);
            println!("{} {identifier} null", token);



            Ok(())
        }
        '0'..='9' => {
            let mut number = "".to_string();
            let mut dot_counter: u8 = 0;
            let mut trailing_dot = false;
            number.push(*char);

            while let Some(&next_char) = chars.peek() {
                if !next_char.is_digit(10) && next_char != '.' || (next_char == '.' && dot_counter > 0) {
                    break;
                }
                if next_char == '.' {
                    dot_counter += 1;
                }

                number.push(chars.next().unwrap())
            }


            // if number.ends_with('.') {
            //     number.pop();
            //     trailing_dot = true
            // }
            //
            let mut number_literal = number.clone();

            if number_literal.ends_with('.') {
                number.pop();
                trailing_dot = true;
                number_literal.extend(['0'])
            }

            if let Some(dot_pos) = number_literal.find('.') {
                let (int_part, frac_part) = number_literal.split_at(dot_pos + 1); // include the dot in int_part
                let trimmed_frac = frac_part.trim_end_matches('0');

                if trimmed_frac.is_empty() {
                    number_literal = format!("{}0", int_part);
                } else {
                    number_literal = format!("{}{}", int_part, trimmed_frac);
                }
            } else {
                number_literal = format!("{}.0", number_literal);
            }


            println!("{} {number} {number_literal}", TokenType::Number);

            if trailing_dot {
                return match_token(&'.', chars, line_number, is_error);
            }

            Ok(())
        }
        '"' => {
            let mut literal = "".to_string();
            let mut is_complete = false;
            while let Some(next_char) = chars.next() {
                if next_char == '"' {
                    is_complete = true;
                    break;
                }

                literal.push(next_char)
            }
            if is_complete {
                println!("{} {double_quote}{literal}{double_quote} {literal}", TokenType::String, double_quote = TokenType::DoubleQuote);
            } else {
                *is_error = true;
                writeln!(io::stderr(), "[line {}] Error: Unterminated string.", line_number + 1).unwrap();
            }

            Ok(())
        }
        ' ' => Ok(()),
        '\t' => Ok(()),
        '(' => {
            println!("{} {char} null", TokenType::LeftParen);
            Ok(())
        }
        ')' => {
            println!("{} {char} null", TokenType::RightParen);
            Ok(())
        }
        '{' => {
            println!("{} {char} null", TokenType::LeftBrace);
            Ok(())
        }
        '}' => {
            println!("{} {char} null", TokenType::RightBrace);
            Ok(())
        }
        '*' => {
            println!("{} {char} null", TokenType::Star);
            Ok(())
        }
        '-' => {
            println!("{} {char} null", TokenType::Minus);
            Ok(())
        }
        '+' => {
            println!("{} {char} null", TokenType::Plus);
            Ok(())
        }
        '.' => {
            println!("{} {char} null", TokenType::Dot);
            Ok(())
        }
        ',' => {
            println!("{} {char} null", TokenType::Comma);
            Ok(())
        }
        ';' => {
            println!("{} {char} null", TokenType::Semicolon);
            Ok(())
        }
        '/' => {
            if let Some(next_char) = chars.next() {
                match next_char {
                    '/' => Err(()),
                    _ => {
                        println!("{} {char} null", TokenType::Less);
                        match_token(&next_char, chars, line_number, is_error)
                    }
                }
            } else {
                println!("{} {char} null", TokenType::Slash);
                Ok(())
            }
        }
        '<' => {
            if let Some(next_char) = chars.next() {
                match next_char {
                    '=' => {
                        println!("{} {char}{next_char} null", TokenType::LessEqual);
                        Ok(())
                    }
                    _ => {
                        println!("{} {char} null", TokenType::Less);
                        let _ = match_token(&next_char, chars, line_number, is_error);
                        Ok(())
                    }
                }
            } else {
                println!("{} {char} null", TokenType::Less);
                Ok(())
            }
        }
        '>' => {
            if let Some(next_char) = chars.next() {
                match next_char {
                    '=' => {
                        println!("{} {char}{next_char} null", TokenType::GreaterEqual);
                        Ok(())
                    }
                    _ => {
                        println!("{} {char} null", TokenType::Greater);
                        let _ = match_token(&next_char, chars, line_number, is_error);
                        Ok(())
                    }
                }
            } else {
                println!("{} {char} null", TokenType::Greater);
                Ok(())
            }
        }

        '!' => {
            if let Some(next_char) = chars.next() {
                match next_char {
                    '=' => {
                        println!("{} {char}{next_char} null", TokenType::BangEqual);
                        Ok(())
                    }
                    _ => {
                        println!("{} {char} null", TokenType::Bang);
                        let _ = match_token(&next_char, chars, line_number, is_error);
                        Ok(())
                    }
                }
            } else {
                println!("{} {char} null", TokenType::Bang);
                Ok(())
            }
        }
        '=' => {
            if let Some(next_char) = chars.next() {
                match next_char {
                    '=' => {
                        println!("{} {char}{next_char} null", TokenType::EqualEqual);
                        Ok(())
                    }
                    _ => {
                        println!("{} {char} null", TokenType::Equal);
                        let _ = match_token(&next_char, chars, line_number, is_error);
                        Ok(())
                    }
                }
            } else {
                println!("{} {char} null", TokenType::Equal);
                Ok(())
            }
        }
        // '=' => println!("{} {char} null", TokenType::Equal),
        _ => {
            if !*is_error {
                *is_error = true
            }
            writeln!(io::stderr(), "[line {}] Error: Unexpected character: {}", line_number + 1, char).unwrap();
            Ok(())
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];


    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });


            if !file_contents.is_empty() {
                let mut is_error = false;
                for (line_number, line) in file_contents.lines().enumerate() {
                    let mut chars = line.chars().peekable();
                    while let Some(char) = chars.next() {
                        match match_token(&char, &mut chars, &line_number, &mut is_error) {
                            Ok(_) => {}
                            Err(_) => break
                        }
                        // chars.next();
                    }
                }
                println!("EOF  null");
                match is_error {
                    true => exit(65),
                    false => exit(0)
                }
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
