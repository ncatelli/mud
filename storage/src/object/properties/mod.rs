extern crate serde;
extern crate serde_json;
extern crate parser;

use std::collections::HashMap;

// Properties stores a key value of object properties.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Properties {
    properties: HashMap<String, parser::lexer::Primitive>
}

impl Properties {
    #[allow(dead_code)]
    pub fn get(self, property: String) -> Option<parser::lexer::Primitive> {
        match self.properties.get(&property) {
            Some(v) => Some(v.clone()),
            None => None
        }
    }

    #[allow(dead_code)]
    pub fn set(mut self, property: String, value: parser::lexer::Primitive) {
        self.properties.insert(property.clone(), value.clone());
    }
}

// Validates that a properties key is valid.
#[allow(dead_code)]
fn validate_property_key(property: &str) -> bool {
    for c in ["\n", " "].iter() {
        if property.contains(c) {
            return false
        }
    }

    true
}