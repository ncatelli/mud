//! lexer includes methods for lexing input strings into internal primitives.
extern crate serde;
extern crate serde_json;

use super::errors::{ErrorKind, ParseError};
use std::iter::Peekable;

#[cfg(test)]
mod tests;

/// Primitive stores all lexer-level primites as returned by the lex command
///
/// # Types
/// - Float is directly represented by rust's f64 type.
/// - Int is directly represented by rust's i64 type.
/// - Str corresponds to any string wrapped in double quotes. Internally this
///   is represented by an owned String.
/// - Symbol corresponds with a one word string and is used to represent
///   commands or keywords.
/// - Nil is currently unused and represents nothing. This may be deprecated.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Primitive {
    Float(f64),
    Int(i64),
    Str(String),
    Symbol(String),
    Nil,
}

impl ToString for Primitive {
    fn to_string(&self) -> String {
        match self {
            Primitive::Float(f) => f.to_string(),
            Primitive::Int(i) => i.to_string(),
            Primitive::Str(s) => s.clone(),
            Primitive::Symbol(sym) => sym.clone(),
            Primitive::Nil => "".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Lexeme {
    Char(char),
    Integer(char),
    DoubleQuote,
    Period,
    Whitespace,
    Error(char),
}

impl ToString for Lexeme {
    fn to_string(&self) -> String {
        match self {
            Lexeme::Char(c) => c.to_string(),
            Lexeme::Integer(c) => c.to_string(),
            Lexeme::DoubleQuote => "\"".to_string(),
            Lexeme::Period => ".".to_string(),
            Lexeme::Whitespace => " ".to_string(),
            Lexeme::Error(c) => format!("Error({})", c).to_string(),
        }
    }
}

/// Read each character into an array of Lexemes.
fn generate_lexeme_vector(input: &String) -> Result<Vec<Lexeme>, ParseError> {
    let lex_vec = input
        .chars()
        .map(|c| match c {
            c if c.is_alphabetic() => Lexeme::Char(c),
            c if c.is_whitespace() => Lexeme::Whitespace,
            c if c.is_digit(10) => Lexeme::Integer(c),
            '"' => Lexeme::DoubleQuote,
            '.' => Lexeme::Period,
            _ => Lexeme::Error(c),
        }).collect();

    Ok(lex_vec)
}

/// The lex function attempts to parse an input string into a vector of
/// primitives after first breaking it into an intermediary lexeme format.
///
/// # Examples
///
/// ```
/// use parser::lexer;
/// use parser::lexer::Primitive;
/// let input = String::from("Hello \"world\" 42 3.14");
///
/// assert_eq!(
///     Ok(
///         vec![
///             Primitive::Symbol("Hello".to_string()),
///             Primitive::Str("world".to_string()),
///             Primitive::Int(42),
///             Primitive::Float(3.14)
///         ]
///     ),
///     lexer::lex(&input)
/// );
/// ```
pub fn lex(input: &String) -> Result<Vec<Primitive>, ParseError> {
    let mut tok_vec: Vec<Primitive> = Vec::new();
    let lexeme_vec = match generate_lexeme_vector(input) {
        Ok(lt) => lt,
        Err(e) => return Err(e),
    };

    let l_iter = lexeme_vec.into_iter();
    let mut lp = l_iter.peekable();
    while let Some(l) = lp.peek().cloned() {
        match l {
            Lexeme::DoubleQuote => match lex_str(&mut lp) {
                Ok(t) => {
                    tok_vec.push(t);
                    continue;
                }
                Err(e) => return Err(e),
            },
            Lexeme::Char(_) => match lex_symbol(&mut lp) {
                Ok(t) => {
                    tok_vec.push(t);
                    continue;
                }
                Err(e) => return Err(e),
            },
            Lexeme::Integer(_) => match lex_number(&mut lp) {
                Ok(t) => {
                    tok_vec.push(t);
                    continue;
                }
                Err(e) => return Err(e),
            },
            Lexeme::Whitespace => {
                lp.next();
            }
            _ => {
                return Err(ParseError::new(
                    ErrorKind::InvalidLexeme,
                    format!("{} is an invalid lexeme.", l.to_string()),
                ))
            }
        }
    }

    Ok(tok_vec)
}

fn lex_str<T: Iterator<Item = Lexeme>>(iter: &mut Peekable<T>) -> Result<Primitive, ParseError> {
    let mut str_vec = String::new();

    // Burn the first doublequote.
    iter.next();

    for val in iter {
        match val {
            Lexeme::DoubleQuote => return Ok(Primitive::Str(str_vec)),
            _ => str_vec.push_str(&val.to_string()),
        }
    }

    Err(ParseError::new(
        ErrorKind::EOI,
        "End of input reached before string termination.".to_string(),
    ))
}

fn lex_number<T: Iterator<Item = Lexeme>>(iter: &mut Peekable<T>) -> Result<Primitive, ParseError> {
    let mut str_vec = String::new();

    while let Some(val) = iter.next() {
        match val {
            Lexeme::Integer(i) => str_vec.push_str(&i.to_string()),
            Lexeme::Period => str_vec.push_str(&String::from(".")),
            Lexeme::Whitespace => break,
            _ => {
                return Err(ParseError::new(
                    ErrorKind::InvalidLexeme,
                    format!("{} is an invalid number.", val.to_string()),
                ))
            }
        }
    }

    if str_vec.contains('.') {
        match str_vec.parse::<f64>() {
            Ok(f) => return Ok(Primitive::Float(f)),
            Err(_) => return Err(ParseError::new(ErrorKind::FloatError, str_vec)),
        }
    } else {
        match str_vec.parse::<i64>() {
            Ok(i) => return Ok(Primitive::Int(i)),
            Err(_) => return Err(ParseError::new(ErrorKind::IntegerError, str_vec)),
        }
    }
}

fn lex_symbol<T: Iterator<Item = Lexeme>>(iter: &mut Peekable<T>) -> Result<Primitive, ParseError> {
    let mut str_vec = String::new();

    for val in iter {
        match val {
            Lexeme::Char(c) => str_vec.push_str(&c.to_string()),
            Lexeme::Whitespace => break,
            _ => {
                return Err(ParseError::new(
                    ErrorKind::InvalidLexeme,
                    format!("{} is an invalid symbol.", val.to_string()),
                ))
            }
        }
    }

    Ok(Primitive::Symbol(str_vec))
}
