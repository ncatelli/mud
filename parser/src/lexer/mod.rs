use std::iter::Peekable;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Clone)]
pub enum Primitive {
    Float(f64),
    Int(i64),
    Str(String),
    Symbol(String),
    Nil,
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
fn generate_lexeme_vector(input: &String) -> Result<Vec<Lexeme>, &'static str> {
    let lex_vec = input
        .chars()
        .map(|c| match c {
            c if c.is_alphabetic() => Lexeme::Char(c),
            c if c.is_whitespace() => Lexeme::Whitespace,
            c if c.is_digit(10) => Lexeme::Integer(c),
            '"' => Lexeme::DoubleQuote,
            '.' => Lexeme::Period,
            _ => Lexeme::Error(c),
        })
        .collect();

    Ok(lex_vec)
}

pub fn lex(input: &String) -> Result<Vec<Primitive>, &'static str> {
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
                Ok(t) => tok_vec.push(t),
                Err(e) => return Err(e),
            },
            Lexeme::Char(_) => match lex_symbol(&mut lp) {
                Ok(t) => tok_vec.push(t),
                Err(e) => return Err(e),
            },
            Lexeme::Integer(_) => match lex_number(&mut lp) {
                Ok(t) => tok_vec.push(t),
                Err(e) => return Err(e),
            },
            Lexeme::Whitespace => continue,
            _ => return Err("Unexpected case"),
        }

        lp.next();
    }

    Ok(tok_vec)
}

fn lex_str<T: Iterator<Item = Lexeme>>(iter: &mut Peekable<T>) -> Result<Primitive, &'static str> {
    let mut str_vec = String::new();

    // Burn the first doublequote.
    iter.next();

    for val in iter {
        match val {
            Lexeme::DoubleQuote => return Ok(Primitive::Str(str_vec)),
            _ => str_vec.push_str(&val.to_string()),
        }
    }

    Err("End of input reached before string termination.")
}

fn lex_number<T: Iterator<Item = Lexeme>>(
    iter: &mut Peekable<T>,
) -> Result<Primitive, &'static str> {
    let mut str_vec = String::new();

    while let Some(val) = iter.next() {
        match val {
            Lexeme::Integer(i) => str_vec.push_str(&i.to_string()),
            Lexeme::Period => str_vec.push_str(&String::from(".")),
            Lexeme::Whitespace => break,
            _ => return Err("Invalid character."),
        }
    }

    if str_vec.contains('.') {
        match str_vec.parse::<f64>() {
            Ok(f) => return Ok(Primitive::Float(f)),
            Err(_) => return Err("Unable to parse float."),
        }
    } else {
        match str_vec.parse::<i64>() {
            Ok(i) => return Ok(Primitive::Int(i)),
            Err(_) => return Err("Unable to parse integer."),
        }
    }
}

fn lex_symbol<T: Iterator<Item = Lexeme>>(
    iter: &mut Peekable<T>,
) -> Result<Primitive, &'static str> {
    let mut str_vec = String::new();

    for val in iter {
        match val {
            Lexeme::Char(c) => str_vec.push_str(&c.to_string()),
            _ => return Err("Ref must contain only characters"),
        }
    }

    Ok(Primitive::Symbol(str_vec))
}
