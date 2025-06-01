use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct Changeable {
    pub server_id: i32,
    pub prefix: String,
    pub lone_word_prob: i32,
    pub consider_frequency: bool,
}

#[derive(Deserialize, Serialize, Debug)]
struct WordEntry {
    pub next_words: HashMap<String, u32>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Markov {
    pub server_id: i32,
    pub lone_words: Vec<String>,
    pub words: HashMap<String, WordEntry>,
}