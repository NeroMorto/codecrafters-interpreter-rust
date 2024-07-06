use std::env;
use std::fmt::{Display, Formatter};
use std::fs;
use std::io::{self, Write};
use std::process::exit;

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
                    for char in line.chars() {
                        match char {
                            '(' => println!("{} {char} null", TokenType::LeftParen),
                            ')' => println!("{} {char} null", TokenType::RightParen),
                            '{' => println!("{} {char} null", TokenType::LeftBrace),
                            '}' => println!("{} {char} null", TokenType::RightBrace),
                            '*' => println!("{} {char} null", TokenType::Star),
                            '-' => println!("{} {char} null", TokenType::Minus),
                            '+' => println!("{} {char} null", TokenType::Plus),
                            '.' => println!("{} {char} null", TokenType::Dot),
                            ',' => println!("{} {char} null", TokenType::Comma),
                            ';' => println!("{} {char} null", TokenType::Semicolon),
                            _ => {
                                if !is_error {
                                    is_error = true
                                }
                                writeln!(io::stderr(), "[line {}] Error: Unexpected character: {}", line_number + 1, char).unwrap() }
                        }
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
