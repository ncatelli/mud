//! lexer includes methods for lexing input strings into internal primitives.
extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;
use object::Object;

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

    let obj: Object = match serde_json::from_str(&raw_object) {
        Ok(obj) => obj,
        Err(e) => panic!(e)
    };

    assert_eq!(Object{
        id: 122,
        name: String::from("Generic Object"),
        description: String::from("This is a generic object"),
        contents: vec![0],
    }, obj);
}