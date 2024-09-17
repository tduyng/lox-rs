use crate::error::ExitCode;

pub mod evaluate;
pub mod parse;
pub mod run;
pub mod tokenize;

pub trait Command {
    fn execute(&self) -> ExitCode;
}
