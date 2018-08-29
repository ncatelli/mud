use lexer::Primitive;

#[derive(Debug)]
pub struct Command {
    verb: Primitive,
    direct_object: Primitive,
    preposition: Primitive,
    indirect_object: Primitive,
}

impl Command {
    pub fn new(v: Primitive, d: Primitive, p: Primitive, io: Primitive) -> Command {
        Command {
            verb: v,
            direct_object: d,
            preposition: p,
            indirect_object: io,
        }
    }
}
