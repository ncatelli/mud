#[macro_use]
extern crate serde_derive;

pub mod command;
pub mod errors;
pub mod lexer;

#[cfg(test)]
mod tests;

use command::Command;

pub fn parse(stmt: String) -> Result<Command, errors::ParseError> {
    match lexer::lex(&stmt) {
        Ok(v) => Command::new(v),
        Err(e) => Err(e),
    }
}
