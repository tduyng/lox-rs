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
                write!(f, "[line {}] Error: Unterminated string", line)
            }
        }
    }
}

impl std::error::Error for ScannerError {}
