extern crate parser;

use std::collections::HashMap;

pub mod verb;
use self::verb::Verb;

#[cfg(test)]
mod tests;

pub type Verbs = HashMap<String, Verb>;
