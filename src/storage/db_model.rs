use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct GuildModel {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) system_channel_id: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct WordEntry {
    pub(crate) next_words: Option<HashMap<String, u32>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Markov {
    pub(crate) words: Option<HashMap<String, WordEntry>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct WordChaining {
    pub(crate) id: u64,
    pub(crate) word_id: u64,
    pub(crate) next_word_id: u64,
    pub(crate) frequency: u32,
    pub(crate) guild_id: u64,
}

impl Markov {
    pub(crate) fn new() -> Self {
        Self { words: None }
    }

    pub(crate) fn from_file(guild_id: u64) -> Result<Self, io::Error> {
        todo!();
    }

    pub(crate) fn add_word(&mut self, current_word: String, next_word: Option<String>) {
        let words_map = self.words.get_or_insert_with(HashMap::new);

        let word_entry: &mut WordEntry = words_map
            .entry(current_word)
            .or_insert_with(|| WordEntry { next_words: None });

        let next_words_map: &mut HashMap<String, u32> =
            word_entry.next_words.get_or_insert_with(HashMap::new);

        *next_words_map
            .entry(next_word.unwrap()) // Use the owned String directly
            .or_insert(0) += 1;
    }
}
