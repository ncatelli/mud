extern crate serde;
extern crate serde_json;

use lexer::Primitive;

#[cfg(test)]
mod tests;

// Command stores the result of a parsed input line.
#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    verb: Option<Primitive>,
    direct_object: Option<Primitive>,
    preposition: Option<Primitive>,
    indirect_object: Option<Primitive>,
}

impl Command {
    // new creates a new instance of command.
    //
    // # Arguments
    // - tokens: takes a vector of lexer::Primitives and attempts to assign
    //   them to their corresponding part of speech. This argument expects
    //   either 1, 2 or 4 elements in the vector.
    //
    // # Examples
    //
    // ```
    // use command::Command;
    // use lexer::Primitive;
    //
    // let input = "go";
    //
    // assert_eq!(
    //   Ok(
    //      Command {
    //        verb: Some(Primitive::Symbol("go".to_string()),
    //        direct_object: None,
    //        preposition: None,
    //        indirect_object: None
    //      },
    //      Command::new(vec![Primitive::Symbol("go".to_string())])
    //   )
    // );
    // ```
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

    // Returns the verb part of speech.
    pub fn verb(&self) -> Option<Primitive> {
        self.verb.clone()
    }

    // Returns the direct object part of speech.
    pub fn direct_object(&self) -> Option<Primitive> {
        self.direct_object.clone()
    }

    // Returns the preposition part of speech.
    pub fn preposition(&self) -> Option<Primitive> {
        self.preposition.clone()
    }

    // Returns the indirect object part of speech.
    pub fn indirect_object(&self) -> Option<Primitive> {
        self.indirect_object.clone()
    }
}

impl ToString for Command {
    fn to_string(&self) -> String {
        let mut output: String = String::new();

        match &self.verb {
            Some(v) => output.push_str(&format!("{}", v.to_string())),
            None => (),
        }

        match &self.direct_object {
            Some(d) => output.push_str(&format!(" {}", d.to_string())),
            None => (),
        }

        match &self.preposition {
            Some(p) => output.push_str(&format!(" {}", p.to_string())),
            None => (),
        }

        match &self.indirect_object {
            Some(io) => output.push_str(&format!(" {}", io.to_string())),
            None => (),
        }

        output
    }
}
