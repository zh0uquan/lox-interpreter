use std::env;
use std::fs;
use std::io::{self, stderr, Write};

mod scanner;
mod token;

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
            // Read file contents
            let file_contents = match fs::read_to_string(filename) {
                Ok(contents) => contents,
                Err(e) => {
                    eprintln!("Failed to read file {}: {}", filename, e);
                    return;
                }
            };

            // Create scanner and process tokens
            if !file_contents.is_empty() {
                let mut scanner = scanner::Scanner::new(file_contents.as_bytes());
                for token in scanner.scan_tokens() {
                    println!("{}", token);
                }
            }
        }
        _ => eprintln!("Unknown command: {}", command),
    }
}
