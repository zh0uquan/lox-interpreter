use std::cell::RefCell;
use std::env;
use std::fs;
use crate::parser::Expr;

use crate::token::{Token, TokenType};

mod enviornment;
mod interpreter;
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

    fn run(&self, command: &str, file_contents: String) {
        if file_contents.is_empty() {
            println!("EOF  null");
            return;
        }
        match command {
            "tokenize" => {
                let mut scanner = scanner::Scanner::new(file_contents.as_bytes(), self);
                let tokens = scanner.scan_tokens();

                for token in tokens {
                    println!("{}", token);
                }
                if *self.has_error.borrow() {
                    std::process::exit(65);
                }
            }
            "parse" => {
                let mut scanner = scanner::Scanner::new(file_contents.as_bytes(), self);
                let tokens = scanner.scan_tokens();

                let parser = parser::Parser::new(tokens, self);
                let parsed_stmts = parser.parse();
                if *self.has_error.borrow() {
                    std::process::exit(65);
                }
                for stmt in parsed_stmts {
                    println!("{}", stmt);
                }
            }
            "evaluate" => {
                let mut scanner = scanner::Scanner::new(file_contents.as_bytes(), self);
                let tokens = scanner.scan_tokens();

                let parser = parser::Parser::new(tokens, self);
                let res = parser.parse();
                for r in res.iter() {
                    println!("{}", r);
                }
                let interpreter = interpreter::Interpreter::new();
                match interpreter.interpret(res) {
                    Ok(exprs) => {
                        exprs.iter().for_each(|expr|
                            println!("{}", expr)
                        );
                    }
                    Err(err) => {
                        println!("{}", err);
                        std::process::exit(70);
                    }
                };
                if *self.has_error.borrow() {
                    std::process::exit(65);
                }
            }
            _ => eprintln!("Unknown command: {}", command),
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
    let file_contents = get_file_contents(filename);
    lox.run(command.as_str(), file_contents);
}
