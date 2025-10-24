use rand::distributions::Distribution;
use crate::storage::db_model::Markov;
use rand::Rng;
use rand::distributions::WeightedIndex;
use rand::prelude::SliceRandom;
use std::io;
use regex::Regex;
use crate::errors::db_error::DbError;
use crate::storage::db_client::DbClient;

pub(crate) async fn tokenize_text(text: String) -> Option<Vec<String>> {
    let re = Regex::new(
        r"(https?://\S+)|(<:[a-zA-Z0-9_]+:[0-9]+>)|([^,.:;'?!@#$%&*(){}\[\]<>/_— ]+)|([,.:;'?!@#$%&*(){}\[\]<>/_—])")
        .unwrap();

    let mut words: Vec<String> = Vec::new();

    for cap in re.captures_iter(text.as_str()) {
        if cap.get(3).is_some() || cap.get(4).is_some() {
            let token = cap.get(0).unwrap().as_str().to_string();
            words.push(token);
        }
    }

    Some(words)
}

pub(crate) async fn store_sentence(words: Vec<String>, db_client: &DbClient) -> Result<(), DbError> {
    for word in &words {
        if let Ok(true) = db_client.is_word_duplicate(word.clone()).await {
            db_client.store_word(word.clone()).await?;
        }
    }

    Ok(())
}

pub(crate) fn generate_text(guild_id: u64) -> Result<String, io::Error> {
    // TODO: make her randomly send a generated sentence here, say... 5% of chance


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
