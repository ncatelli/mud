extern crate serde;
extern crate serde_json;
extern crate parser;

use std::collections::HashMap;

#[cfg(test)]
mod tests;

// Properties stores a key value of object properties.
pub type Properties = HashMap<String, parser::lexer::Primitive>;

// Validates that a properties key is valid.
#[allow(dead_code)]
pub fn validate_property_key(property: &str) -> bool {
    for c in ["\n", " "].iter() {
        if property.contains(c) {
            return false
        }
    }

    true
}