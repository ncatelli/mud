use std::error;
use std::fmt;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
enum ErrorKind {
    InvalidChar,
    OpenQuote,
}

#[derive(Debug, PartialEq)]
struct ParseError {
    error_kind: ErrorKind,
    line: String,
    column: u32,
}

impl ParseError {
    fn new(ek: ErrorKind, l: String, c: u32) -> ParseError {
        ParseError {
            error_kind: ek,
            line: l,
            column: c,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Shit broke")
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        "Shit broke"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
