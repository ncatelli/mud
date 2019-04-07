#[cfg(test)]
mod tests;

extern crate parser;

pub enum Event {
    Player(parser::command::Command)
}