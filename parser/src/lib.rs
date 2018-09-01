pub mod command;
pub mod lexer;

#[cfg(test)]
mod tests;

use command::Command;

pub fn parse(stmt: String) -> Result<Command, &'static str> {
    match lexer::lex(&stmt) {
        Ok(v) => Command::new(v),
        Err(e) => Err(e),
    }
}
