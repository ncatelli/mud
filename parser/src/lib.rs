mod command;
mod lexer;
#[cfg(test)]
mod tests;

//use lexer::{Primitive, Token};

pub fn parse(stmt: String) -> Result<command::Command, &'static str> {
    match lexer::lex(&stmt) {
        Ok(_) => Err("passed"),
        Err(e) => Err(e),
    }
}
