use std::{env, io};
use std::fs;

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

    match command.as_str() {
        "tokenize" => {
            // Read file contents
            // let file_contents = match fs::read_to_string(filename) {
            //     Ok(contents) => contents,
            //     Err(e) => {
            //         eprintln!("Failed to read file {}: {}", filename, e);
            //         String::new()
            //     }
            // };
            
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            // Create scanner and process tokens
            if file_contents.is_empty() {
                eprintln!("EOF  null")
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
        _ => eprintln!("Unknown command: {}", command),
    }
}
