use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;


pub(crate) struct MarkovTable {
    pub(crate) table: HashMap<String, HashMap<String, u32>>,
}

pub(crate) struct Sentence {
    pub(crate) sentence: String,
    pub(crate) words: Vec<String>,
}

impl Sentence {

}

impl MarkovTable {

}

impl fmt::Display for MarkovTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.table)
    }
}