use std::iter::Peekable;

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
    Integer(char),
    Period,
    Semicolon,
    Colon,
    LeftParen,
    RightParen,
    DoubleQuote,
    LeftBracket,
    RightBracket,
    Pound,
    Whitespace,
    Error(char),
}

impl ToString for Lexeme {
	fn to_string(&self) -> String {
		match self {
			Lexeme::Char(c) => c.to_string(),
			Lexeme::Integer(c) => c.to_string(),
			Lexeme::Period => ".".to_string(),
			Lexeme::Semicolon => ";".to_string(),
			Lexeme::Colon => ":".to_string(),
			Lexeme::LeftParen => "(".to_string(),
			Lexeme::RightParen => ")".to_string(),
			Lexeme::DoubleQuote => "\"".to_string(),
			Lexeme::LeftBracket => "[".to_string(),
			Lexeme::RightBracket => "]".to_string(),
			Lexeme::Pound => "#".to_string(),
			Lexeme::Whitespace => " ".to_string(),
			Lexeme::Error(c) => "Error(c)".to_string(),
		}
	}
}

/// Read each character into an array of Lexemes.
fn generate_lexeme_vector(input: &str) -> Result<Vec<Lexeme>, &'static str> {
    let lex_vec = input.chars().map(|c| {
        match c {
            c if c.is_alphabetic() => Lexeme::Char(c),
            c if c.is_whitespace() => Lexeme::Whitespace,
            c if c.is_digit(10) => Lexeme::Integer(c),
            '.'  => Lexeme::Period,
            ':'  => Lexeme::Colon,
            ';'  => Lexeme::Semicolon,
            '('  => Lexeme::LeftParen,
            ')'  => Lexeme::RightParen,
            '['  => Lexeme::LeftBracket,
            ']'  => Lexeme::RightBracket,
            '"'  => Lexeme::DoubleQuote,
            '#'  => Lexeme::Pound,
            _    => Lexeme::Error(c),
        }
    }).collect();

    Ok(lex_vec)
}

fn lex<'a>(input: &str) -> Result<Vec<Token<'a>>, &'static str> {
    let tok_vec: Vec<Token> = Vec::new();
    let lexeme_vec = match generate_lexeme_vector(&input) {
       Ok(lt) => lt,
       Err(e) => return Err(e),
    };

    let mut lp = lexeme_vec.iter().peekable();
    while let Some(&l) = lp.peek() {
        match l {
            _ => (),
        }
    }

    Err("Lexer Fails")
}
