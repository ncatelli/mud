use lexer::Token;

#[derive(Debug)]
pub struct Command {
    verb: Token,
    direct_object: Token,
    preposition: Token,
    indirect_object: Token,
}

impl Command {
    pub fn new(v: Token, d: Token, p: Token, io: Token) -> Command {
        Command {
            verb: v,
            direct_object: d,
            preposition: p,
            indirect_object: io,
        }
    }
}
