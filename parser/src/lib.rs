mod command;
mod lexer;

pub fn parse(_stmt: String) -> Result<command::Command, &'static str> {
    Err("Invalid Parse")
}
