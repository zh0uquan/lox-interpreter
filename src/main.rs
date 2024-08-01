use std::cell::RefCell;
use std::env;
use std::fs;

use crate::token::{Token, TokenType};

mod parser;
mod scanner;
mod token;

struct Lox {
    has_error: RefCell<bool>,
}

impl Lox {
    fn new() -> Self {
        Lox {
            has_error: RefCell::new(false),
        }
    }
}

impl Lox {
    fn report(&self, line: usize, _where: &str, message: String) {
        *self.has_error.borrow_mut() = true;
        eprintln!("[line {}] Error: {}{}", line, _where, message);
    }

    fn error(&self, token: &Token, message: String) {
        if token.token_type == TokenType::EOF {
            self.report(token.line, " at end ", message);
        } else {
            let lexeme_str = String::from_utf8_lossy(token.lexeme);
            self.report(
                token.line,
                format!(" at '{}' ", lexeme_str).as_str(),
                message,
            );
        }
    }

    fn run(&self, command: &str, get_file_content_func: impl Fn() -> String) {
        match command {
            "tokenize" => {
                let file_contents = get_file_content_func();
                if file_contents.is_empty() {
                    println!("EOF  null")
                } else {
                    let mut scanner = scanner::Scanner::new(file_contents.as_bytes(), &self);
                    let tokens = scanner.scan_tokens();

                    for token in tokens {
                        println!("{}", token);
                    }
                }
            }
            "parse" => {
                let file_contents = get_file_content_func();
                if file_contents.is_empty() {
                    println!("EOF  null")
                } else {
                    let mut scanner = scanner::Scanner::new(file_contents.as_bytes(), &self);
                    let tokens = scanner.scan_tokens();

                    let mut parser = parser::Parser::new(tokens, &self);
                    println!("{}", parser.parse());
                }
            }
            _ => eprintln!("Unknown command: {}", command),
        }
        if *self.has_error.borrow() {
            std::process::exit(65);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let get_file_contents = |filename: &String| {
        fs::read_to_string(filename).unwrap_or_else(|_| {
            eprintln!("Failed to read file {}", filename);
            String::new()
        })
    };

    let lox = Lox::new();

    lox.run(command.as_str(), || get_file_contents(filename));
}
