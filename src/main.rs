use std::env;
use std::fmt::{Display, Formatter};
use std::fs;
use std::io::{self, Write};
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
            TokenType::Slash => write!(f, "SLASH")
        }
    }
}

fn match_token(char: &char, chars: &mut Chars, line_number: &usize, is_error: &mut bool) -> Result<(), ()> {
    match char {
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
                    let mut chars = line.chars();
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
