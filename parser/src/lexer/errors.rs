
enum ParseError {
    InvalidChar,
    OpenQuote,
}

#[derive(Debug)]
struct ParseError {
    ErrorKind: ParseError,
    Line: String,
    Column: u32
}

impl ParseError {
    fn new(ek: ParseError, l: String, c: u32) -> ParseError {
        ParseError {
            ErrorKind: ek,
            Line: l,
            Column: c
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Shit broke")
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        "Shit broke"
    }
}
