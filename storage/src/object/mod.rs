extern crate serde;
extern crate serde_json;
extern crate parser;

use std::collections::HashMap;

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
    properties: HashMap<String, parser::lexer::Primitive>
}

impl Object {
    #[allow(dead_code)]
    fn id(self) -> parser::lexer::Primitive {
        self.id
    }

    #[allow(dead_code)]
    fn name(self) -> parser::lexer::Primitive {
        self.name
    }

    #[allow(dead_code)]
    fn description(self) -> parser::lexer::Primitive {
        self.description
    }

    #[allow(dead_code)]
    fn contents(self) -> Vec<parser::lexer::Primitive> {
        self.contents
    }

    #[allow(dead_code)]
    fn get_property(self, property: String) -> Option<parser::lexer::Primitive> {
        match self.properties.get(&property) {
            Some(v) => Some(v.clone()),
            None => None
        }
    }

    #[allow(dead_code)]
    fn set_property(mut self, property: String, value: parser::lexer::Primitive) {
        self.properties.insert(property.clone(), value.clone());
    }
}