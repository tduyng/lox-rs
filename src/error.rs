use std::fmt;

#[derive(Debug)]
pub enum ScannerError {
    UnexpectedCharacter(char, usize),
    UnterminatedString(usize),
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScannerError::UnexpectedCharacter(c, line) => {
                write!(f, "[line {}] Error: Unexpected character: {}", line, c)
            }
            ScannerError::UnterminatedString(line) => {
                write!(f, "[line {}] Error: Unterminated string.", line)
            }
        }
    }
}

impl std::error::Error for ScannerError {}

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
