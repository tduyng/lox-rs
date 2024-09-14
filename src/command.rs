use crate::error::ExitCode;

pub mod tokenize;

pub trait Command {
    fn execute(&self) -> ExitCode;
}
