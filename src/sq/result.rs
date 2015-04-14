extern crate rustbox;
use rustbox::Color;
use rustbox::RustBox;

use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Clone)]
pub struct Result {
    pub string: String,
    pub matches: BTreeSet<usize>,
}

impl Result {
    pub fn new(s: &str, p: BTreeSet<usize>) -> Result {
        Result { string: s.to_string(), matches: p }
    }
}
