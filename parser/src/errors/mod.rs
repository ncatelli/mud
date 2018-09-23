//! errors includes parse error types.
extern crate serde;
extern crate serde_json;

use std::error;
use std::fmt;

#[cfg(test)]
mod tests;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ErrorKind {
    InvalidLexeme,
    InvalidTokenCount,
    IntegerError,
    FloatError,
    EOI,
}

impl ErrorKind {
    fn to_str(&self) -> &str {
        match self {
            ErrorKind::InvalidLexeme => "Invalid Lexeme",
            ErrorKind::InvalidTokenCount => "Invalid Token Count",
            ErrorKind::IntegerError => "Invalid Integer",
            ErrorKind::FloatError => "Invalid Float",
            ErrorKind::EOI => "End Of Input",
        }
    }
}

impl ToString for ErrorKind {
    fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ParseError {
    error_kind: ErrorKind,
    cause: String,
}

impl ParseError {
    pub fn new(ek: ErrorKind, cause: String) -> ParseError {
        ParseError {
            error_kind: ek,
            cause: cause,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.error_kind.to_string(), self.cause)
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        self.error_kind.to_str()
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
