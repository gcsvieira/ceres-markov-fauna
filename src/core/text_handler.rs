use rand::distributions::Distribution;
use crate::storage::guild_markov_model::Markov;
use crate::utils::file_utils::FileOperations;
use rand::Rng;
use rand::distributions::WeightedIndex;
use rand::prelude::SliceRandom;
use std::io;

pub(super) fn store_text(text: String, guild_id: u64) -> Result<(), io::Error> {
    // TODO: make her randomly send a generated sentence here, say... 5% of chance

    // if the received text is empty, return None
    if text.is_empty() {
        return Ok(());
    }

    let mut markov = match Markov::from_file(guild_id) {
        Ok(markov) => markov,
        Err(e) => return Err(e),
    };

    let words: Vec<String> = text.split_whitespace().map(String::from).collect();

    // TODO: check for links, duplicated words and emotes she can't use. They shouldn't be added.
    // TODO: Merge lone_words with words

    for i in 0..(words.len() - 1) {
        let current_word = words[i].clone();
        let next_word = words[i + 1].clone();

        todo!();
        markov.add_word(current_word, Option::from(next_word));
    }

    todo!();
}

pub(crate) fn generate_text(guild_id: u64) -> Result<String, io::Error> {
    let mut rng = rand::thread_rng();
    let sentence_length = rng.gen_range(6..30);
    let markov = Markov::from_file(guild_id)?;
    let mut generated_text: Vec<String> = Vec::new();

    let words_map = markov.words.as_ref().unwrap();
    let initial_word_candidates: Vec<&String> = words_map.keys().collect();

    let mut current_word = match initial_word_candidates.choose(&mut rng) {
        Some(word) => (*word).clone(),
        None => String::new(),
    };

    for _ in 0..sentence_length {
        generated_text.push(current_word.clone());

        let word_entry = words_map.get(&current_word);

        let next_words_map = match word_entry.and_then(|entry| entry.next_words.as_ref()) {
            Some(next_words_map) => next_words_map,
            None => break,
        };

        let mut candidates: Vec<&String> = Vec::new();
        let mut weights: Vec<u32> = Vec::new();

        for (word, count) in next_words_map.iter() {
            candidates.push(word);
            weights.push(*count);
        }

        if candidates.is_empty() {
            break;
        }

        let dist = match WeightedIndex::new(&weights) {
            Ok(d) => d,
            Err(_) => {
                break;
            }
        };

        let next_word_index = dist.sample(&mut rng);
        current_word = candidates[next_word_index].clone();
    }

    Ok(generated_text.join(" "))
}
