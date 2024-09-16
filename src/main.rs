use std::env;
use std::fs;
use std::process;

use command::evaluate::EvaluateCommand;
use command::tokenize::TokenizeCommand;
use command::Command;

mod command;
mod error;
mod interpreter;
mod parser;
mod scanner;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command_name = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    let command: Box<dyn Command> = match command_name.as_str() {
        "tokenize" => Box::new(TokenizeCommand::new(file_contents)),
        "evaluate" => Box::new(EvaluateCommand::new(file_contents)),
        _ => {
            eprintln!("Unknown command: {}", command_name);
            process::exit(64);
        }
    };

    let exit_code = command.execute();
    process::exit(exit_code.code());
}
