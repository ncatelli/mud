extern crate serde;
extern crate serde_json;

// Object stores the basic object that all game objects willb e serialized 
// into.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Object {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub contents: Vec<u64>,
}