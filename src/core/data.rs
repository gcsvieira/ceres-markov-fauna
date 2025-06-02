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
    pub(crate) fn divide(&mut self) -> () {
        self.words
            .push(self.sentence
                .split_whitespace()
                .collect());
    }
}

impl MarkovTable {
    pub(crate) fn add(&mut self, words: Vec<String>) -> () {
        for i in 0..words.len() - 1 {
            let current_word = words[i].clone();
            let next_word = words[i + 1].clone();
            let next_word_counts = self.table
                .entry(current_word)
                .or_insert_with(HashMap::new);
        }
    }
}

impl fmt::Display for MarkovTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.table)
    }
}