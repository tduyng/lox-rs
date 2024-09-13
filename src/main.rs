use std::env;
use std::fs;
use std::process;

use scanner::Scanner;

mod error;
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

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    match command.as_str() {
        "tokenize" => {
            let mut scanner = Scanner::new(file_contents);
            let tokens = scanner.scan_tokens();

            for token in tokens {
                let token_type = token.token_type.to_string();
                let lexeme = &token.lexeme;

                let literal_str = match &token.literal {
                    Some(value) => value.clone(),
                    None => "null".to_string(),
                };
                println!("{} {} {}", token_type, lexeme, literal_str);
            }
            if scanner.had_errors() {
                process::exit(65);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            process::exit(64)
        }
    }
}
