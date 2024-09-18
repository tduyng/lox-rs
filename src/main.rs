use std::env;
use std::fs;
use std::process;

use command::evaluate::EvaluateCommand;
use command::parse::ParseCommand;
use command::run::RunCommand;
use command::tokenize::TokenizeCommand;
use command::Command;
use error::LoxError;

mod ast;
mod command;
mod environment;
mod error;
mod interpreter;
mod parser;
mod scanner;
mod token;
mod utils;

fn main() -> Result<(), LoxError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return Ok(());
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
        "parse" => Box::new(ParseCommand::new(file_contents)),
        "run" => Box::new(RunCommand::new(file_contents)),
        _ => {
            eprintln!("Unknown command: {}", command_name);
            process::exit(64);
        }
    };

    let exit_code = command.execute()?;
    process::exit(exit_code.code());
}
