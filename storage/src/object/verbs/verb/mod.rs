extern crate serde;
extern crate serde_json;
extern crate parser;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Verb {
    pub verb: Vec<parser::lexer::Primitive>,
    pub direct_object: Vec<parser::lexer::Primitive>,
    pub preposition: Vec<parser::lexer::Primitive>,
    pub indirect_object: Vec<parser::lexer::Primitive>
}

impl Verb {
    pub fn new() -> Verb {
        Verb {
            verb: Vec::new(),
            direct_object: Vec::new(),
            preposition: Vec::new(), 
            indirect_object: Vec::new()
        }
    }
}