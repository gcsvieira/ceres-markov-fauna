use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Config {
    pub(crate) server_id: i32,
    pub(crate) prefix: String,
    pub(crate) command_indicator: char,
    pub(crate) lone_word_prob: i32,
    pub(crate) consider_frequency: bool,
}

#[derive(Deserialize, Serialize, Debug)]
struct WordEntry {
    pub(crate) next_words: HashMap<String, u32>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Markov {
    pub(crate) server_id: i32,
    pub(crate) lone_words: Vec<String>,
    pub(crate) words: HashMap<String, WordEntry>,
}