use std::fmt;

#[derive(Debug)]
pub struct LoxError {
    pub message: String,
    pub line: usize,
}

impl LoxError {
    pub fn new(message: &str, line: usize) -> Self {
        LoxError {
            message: message.to_string(),
            line,
        }
    }
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] Error: {}", self.line, self.message)
    }
}

impl std::error::Error for LoxError {}

#[derive(Debug)]
pub struct ExitCode(i32);

#[allow(unused)]
impl ExitCode {
    pub fn new(code: i32) -> Self {
        ExitCode(code)
    }

    pub fn code(&self) -> i32 {
        self.0
    }
}
