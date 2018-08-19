#[cfg(test)]
mod tests;

#[derive(Debug)]
enum Token<'a> {
    Operator(&'a Operator),
    Operand(&'a ComplexType<'a>),
    Call,
    Args,
}

#[derive(Debug)]
enum ComplexType<'a> {
    Array(Vec<&'a Primitive<'a>>),
    Primitive(&'a Primitive<'a>),
    ObjectId(&'a str),
}

#[derive(Debug)]
enum Operator {
    Op(char),
}

#[derive(Debug)]
enum Primitive<'a> {
    Float(f64),
    Int(i64),
    Str(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum Lexeme {
    Char(char),
    Integer(u32),
    Period,
    Semicolon,
    Colon,
    LeftParen,
    RightParen,
    SingleQuote,
    DoubleQuote,
    Pound,
    Error(char),
}

/// Lexes each character into an array of Lexemes.
fn lex(input: &str) -> Result<Vec<Lexeme>, &'static str> {
    let lex_vec = input.chars().map(|c| {
        match c {
            c if c.is_alphabetic() => Lexeme::Char(c),
            c if c.is_digit(10) => Lexeme::Integer(c.to_digit(10).unwrap()),
            '.'  => Lexeme::Period,
            ':'  => Lexeme::Colon,
            ';'  => Lexeme::Semicolon,
            '('  => Lexeme::LeftParen,
            ')'  => Lexeme::RightParen,
            '\'' => Lexeme::SingleQuote,
            '"'  => Lexeme::DoubleQuote,
            '#'  => Lexeme::Pound,
            _    => Lexeme::Error(c),
        }
    }).collect();

    Ok(lex_vec)
}
