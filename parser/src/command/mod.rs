use lexer::Primitive;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Command {
    verb: Option<Primitive>,
    direct_object: Option<Primitive>,
    preposition: Option<Primitive>,
    indirect_object: Option<Primitive>,
}

impl Command {
    pub fn new(tokens: Vec<Primitive>) -> Result<Command, &'static str> {
        match tokens.len() {
            1 => Ok(Command {
                verb: Some(tokens[0].clone()),
                direct_object: None,
                preposition: None,
                indirect_object: None,
            }),
            2 => Ok(Command {
                verb: Some(tokens[0].clone()),
                direct_object: Some(tokens[1].clone()),
                preposition: None,
                indirect_object: None,
            }),
            4 => Ok(Command {
                verb: Some(tokens[0].clone()),
                direct_object: Some(tokens[1].clone()),
                preposition: Some(tokens[2].clone()),
                indirect_object: Some(tokens[3].clone()),
            }),
            _ => Err("Invalid number of tokens"),
        }
    }

    pub fn verb(&self) -> Option<Primitive> {
        self.verb.clone()
    }

    pub fn direct_object(&self) -> Option<Primitive> {
        self.direct_object.clone()
    }

    pub fn preposition(&self) -> Option<Primitive> {
        self.preposition.clone()
    }

    pub fn indirect_object(&self) -> Option<Primitive> {
        self.indirect_object.clone()
    }
}
