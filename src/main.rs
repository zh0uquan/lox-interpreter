use std::env;
use std::fs;

mod parser;
mod scanner;
mod token;

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

    match command.as_str() {
        "tokenize" => {
            let file_contents = get_file_contents(filename);
            if file_contents.is_empty() {
                println!("EOF  null")
            } else {
                let mut scanner = scanner::Scanner::new(file_contents.as_bytes());
                let (tokens, has_err) = scanner.scan_tokens();

                for token in tokens {
                    println!("{}", token);
                }
                if has_err {
                    std::process::exit(65);
                }
            }
        }
        "parse" => {
            let file_contents = get_file_contents(filename);
            if file_contents.is_empty() {
                println!("EOF  null")
            } else {
                let mut scanner = scanner::Scanner::new(file_contents.as_bytes());
                let (tokens, has_err) = scanner.scan_tokens();
                if has_err {
                    std::process::exit(65);
                }

                let mut parser = parser::Parser::new(tokens);
                println!("{}", parser.parse());
            }
        }
        _ => eprintln!("Unknown command: {}", command),
    }
}
