use std::error;
use std::fmt;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    InvalidChar,
    OpenQuote,
}

impl ToString for ErrorKind {
    fn to_string(&self) -> String {
        match self {
            ErrorKind::InvalidChar => "Invalid Char".to_string(),
            ErrorKind::OpenQuote => "Open Quote".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseError {
    error_kind: ErrorKind,
    line: String,
}

impl ParseError {
    pub fn new(ek: ErrorKind, l: String) -> ParseError {
        ParseError {
            error_kind: ek,
            line: l,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.error_kind.to_string(), self.line)
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        match self.error_kind {
            ErrorKind::InvalidChar => "Invalid char",
            ErrorKind::OpenQuote => "Open quote",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
