use crate::core::db_client::DbClient;
use crate::errors::db_error::DbError;
use log::error;
use rand::Rng;
use regex::Regex;
use std::clone::Clone;

pub(crate) async fn tokenize_text(text: String) -> Option<Vec<String>> {
    let re = Regex::new(
        r"(https?://\S+)|(<@[0-9]+>)|(<:[a-zA-Z0-9_]+:[0-9]+>)|([^,.:;?!@#$%&*(){}\[\]<>/_— \r\n]+)|([,.:;?!@#$%&*(){}\[\]<>/_—\r\n])")
        .unwrap();

    let mut words: Vec<String> = Vec::new();

    for cap in re.captures_iter(text.as_str()) {
        if cap.get(4).is_some() || cap.get(5).is_some() {
            let token = cap.get(0).unwrap().as_str().to_string();

            if token.is_empty() {
                continue;
            }

            if token == "\n" || token == "\r" || token == "\r\n" {
                continue;
            }

            words.push(token);
        }
    }

    Some(words)
}

pub(crate) async fn store_sentence(
    words: Vec<String>,
    guild_id: u64,
    db_client: &DbClient,
) -> Result<(), DbError> {
    let vec_for_chaining = words.clone();
    let vec_len = words.len();

    if words.is_empty() {
        return Ok(());
    }

    for word in words {
        if let Err(why) = db_client.store_word(word.clone()).await {
            error!("[{}] Error storing word: {}", guild_id, why);
        }
    }

    if vec_len > 1 {
        for i in 0..(vec_for_chaining.len() - 1) {
            let current_word = vec_for_chaining[i].clone();
            let next_word = vec_for_chaining[i + 1].clone();

            if let Err(why) = db_client
                .store_word_chaining(guild_id, current_word, next_word)
                .await
            {
                error!("[{}] Failed to store word chaining: {}", guild_id, why)
            }
        }
    } else {
        if let Err(why) = db_client
            .store_word_chaining(guild_id, vec_for_chaining[0].clone(), String::new())
            .await
        {
            error!("[{}] Failed to store word chaining: {}", guild_id, why)
        }
    }

    Ok(())
}

// pub(crate) fn generate_sentence(guild_id: u64, db_client: &DbClient) -> Result<String, io::Error> {
//     // TODO: make her randomly send a generated sentence here, say... 5% of chance
//     let mut rng = rand::thread_rng();
//     let sentence_length = rng.gen_range(6..30);
//     let mut generated_text: Vec<String> = Vec::new();
//
//     let words_map = db_client.extract_words(guild_id);
//
//
//     let initial_word_candidates: Vec<&String> = words_map.keys().collect();
//
//     let mut current_word = match initial_word_candidates.choose(&mut rng) {
//         Some(word) => (*word).clone(),
//         None => String::new(),
//     };
//
//     for _ in 0..sentence_length {
//         generated_text.push(current_word.clone());
//
//         let word_entry = words_map.get(&current_word);
//
//         let next_words_map = match word_entry.and_then(|entry| entry.next_words.as_ref()) {
//             Some(next_words_map) => next_words_map,
//             None => break,
//         };
//
//         let mut candidates: Vec<&String> = Vec::new();
//         let mut weights: Vec<u32> = Vec::new();
//
//         for (word, count) in next_words_map.iter() {
//             candidates.push(word);
//             weights.push(*count);
//         }
//
//         if candidates.is_empty() {
//             break;
//         }
//
//         let dist = match WeightedIndex::new(&weights) {
//             Ok(d) => d,
//             Err(_) => {
//                 break;
//             }
//         };
//
//         let next_word_index = dist.sample(&mut rng);
//         current_word = candidates[next_word_index].clone();
//     }
//
//     Ok(generated_text.join(" "))
// }
