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
    ObjectId(&'a str),
}

#[derive(Debug, PartialEq)]
enum Lexeme {
    Char(char),
    Integer(u32),
    Period,
    Semicolon,
    Colon,
    LeftParen,
    RightParen,
    SingleQuote,
    DoubleQuote,
    LeftBracket,
    RightBracket,
    Pound,
    Whitespace,
    Error(char),
}

/// Read each character into an array of Lexemes.
fn generate_lexeme_vector(input: &str) -> Result<Vec<Lexeme>, &'static str> {
    let lex_vec = input.chars().map(|c| {
        match c {
            c if c.is_alphabetic() => Lexeme::Char(c),
            c if c.is_whitespace() => Lexeme::Whitespace,
            c if c.is_digit(10) => Lexeme::Integer(c.to_digit(10).unwrap()),
            '.'  => Lexeme::Period,
            ':'  => Lexeme::Colon,
            ';'  => Lexeme::Semicolon,
            '('  => Lexeme::LeftParen,
            ')'  => Lexeme::RightParen,
            '['  => Lexeme::LeftBracket,
            ']'  => Lexeme::RightBracket,
            '\'' => Lexeme::SingleQuote,
            '"'  => Lexeme::DoubleQuote,
            '#'  => Lexeme::Pound,
            _    => Lexeme::Error(c),
        }
    }).collect();

    Ok(lex_vec)
}

fn lex<'a>(input: &str) -> Result<Vec<Token<'a>>, &'static str> {
    Err("Lexer Fails")
}
