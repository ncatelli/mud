extern crate serde;
extern crate serde_json;
extern crate parser;

pub mod properties;

#[cfg(test)]
mod tests;

// Object stores the basic object that all game objects will be serialized 
// into.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Object {
    id: parser::lexer::Primitive,
    name: parser::lexer::Primitive,
    description: parser::lexer::Primitive,
    contents: Vec<parser::lexer::Primitive>,
    properties: properties::Properties
}

impl Object {
    pub fn id(self) -> parser::lexer::Primitive {
        self.id
    }

    pub fn name(self) -> parser::lexer::Primitive {
        self.name
    }

    pub fn description(self) -> parser::lexer::Primitive {
        self.description
    }

    pub fn contents(self) -> Vec<parser::lexer::Primitive> {
        self.contents
    }
}