use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process::exit;

use scanner::Scanner;

use crate::parser::Parser;

mod scanner;
mod token;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            // writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let mut scanner = Scanner::new(&file_contents);
            scanner.scan();
            scanner.write_tokens(io::stdout()).unwrap();
            scanner.write_errors(io::stderr()).unwrap();

            match scanner.has_errors() {
                false => exit(0),
                true => exit(65),
            }
        }
        "parse" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });
            let mut scanner = Scanner::new(&file_contents);
            scanner.scan();

            let mut parser = Parser::new();


            parser.parse_peekable(&scanner.tokens);
            match parser.errors.len() {
                0 => {
                    parser.print_expressions(io::stdout()).unwrap();
                    exit(0)
                }
                _ => {
                    for error in &parser.errors {
                        writeln!(io::stderr(), "{error}").unwrap()
                    }
                    exit(65)
                }
            }



        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
