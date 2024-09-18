use crate::error::{ExitCode, LoxError};

pub mod evaluate;
pub mod parse;
pub mod run;
pub mod tokenize;

pub trait Command {
    fn execute(&self) -> Result<ExitCode, LoxError>;
}
