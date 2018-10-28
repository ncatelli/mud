//! lexer includes methods for lexing input strings into internal primitives.
extern crate serde;
extern crate serde_json;
extern crate parser;

use std::fs::File;
use std::io::prelude::*;
use object::Object;
use object::properties::Properties;
use object::verbs::verb::Verb;
use object::verbs::Verbs;


const EXAMPLE_OBJECT_FILE_PATH: &'static str = "/src/object/tests/example_object.json";

// Reads an example json object for testing serialization.
fn read_example_object() -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let mut f = File::open(format!("{}{}", manifest_dir, EXAMPLE_OBJECT_FILE_PATH))
        .expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    return contents
}

#[test]
fn can_deserialize_json_object() {
    let raw_object = read_example_object();

    let mut prop_map = Properties::new();
    prop_map.insert(
        "example property".to_string(),
        parser::lexer::Primitive::Str("example".to_string())
    );

    let mut example_verb = Verb::new();
    example_verb.verb.push(parser::lexer::Primitive::Symbol("move".to_string()));
    example_verb.direct_object.push(parser::lexer::Primitive::Symbol("any".to_string()));
    example_verb.preposition.push(parser::lexer::Primitive::Symbol("from".to_string()));
    example_verb.preposition.push(parser::lexer::Primitive::Symbol("to".to_string()));
    example_verb.indirect_object.push(parser::lexer::Primitive::Symbol("any".to_string()));

    let mut verb_map = Verbs::new();
    verb_map.insert("move".to_string(), example_verb);

    let obj: Object = match serde_json::from_str(&raw_object) {
        Ok(obj) => obj,
        Err(e) => panic!(e)
    };
  
    assert_eq!(Object{
        id: parser::lexer::Primitive::Int(122),
        name: parser::lexer::Primitive::Str("Generic Object".to_string()),
        description: parser::lexer::Primitive::Str("This is a generic object".to_string()),
        contents: vec![parser::lexer::Primitive::Int(0)],
        properties: prop_map,
        verbs: verb_map,
    }, obj);
}
