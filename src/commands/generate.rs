use rand::prelude::IndexedRandom;
use rand::Rng;
use crate::{utils, Data};
use crate::storage::db_model::WordChaining;
use utils::punctuation_utils::get_punctuation_map;
use crate::utils::punctuation_utils::Attach;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command)]
pub(crate) async fn generate(
    ctx: Context<'_>,
    #[description = "How many sentences do you want me to generate? (1 to 10!)"] quantity: Option<
        u8,
    >,
) -> Result<(), Error> {
    if let Some(q) = quantity {
        if q > 10 {
            ctx.say("I can only generate up to 10 sentences at once!")
                .await?;
            return Ok(());
        }
    }

    if let Some(0) = quantity {
        ctx.say("Very funny!! Are you playing with me? I won't generate anything then! Hmpf!")
            .await?;
        return Ok(())
    }

    match quantity {
        None | Some(1) => {
            let option_sentence = generate_sentence(&ctx).await;
            if let Some(sentence) = option_sentence {
                ctx.say(sentence).await?;
            }
            Ok(())
        },
        Some(quantity) => {
            let mut sentences = String::new();

            for _ in 0..quantity {
                let sentence = match generate_sentence(&ctx).await {
                    Some(sentence) => sentence,
                    None => continue
                };
                sentences = format!("{}\n{}", sentences, sentence)
            }

            if !sentences.is_empty() { ctx.say(sentences).await?; }
            Ok(())
        }
    }
}

async fn generate_sentence(ctx: &Context<'_>) -> Option<String> {
    let sentence_length: u8 = rand::random_range(6..30);
    let mut generated_text: Vec<String> = Vec::new();

    // extract all word chainings related to the server
    let extracted_words: Vec<WordChaining> = match ctx.data().db
        .extract_words(ctx.guild_id()
            .unwrap() // using unwrap because we wouldn't be here if there wasn't one
            .get())
        .await {
        Ok(words) => words,
        Err(e) => return None
    };

    // choose a random initial word id
    let mut current_word_id = match extracted_words.choose(&mut rand::rng()) {
        Some(word) => word.word_id,
        None => return None,
    };

    for _ in 0..sentence_length {
        // get word pretty and add it to generated_text
        match ctx.data().db.get_word_pretty(&current_word_id).await {
            Ok(word_pretty) => {
                if let Some(w_c) = word_pretty {
                    generated_text.push(w_c);
                }
            }
            Err(_) => continue
        };

        // get all next word ids related to current word id
        let next_words = match ctx.data().db
            .get_next_words(&current_word_id, ctx.guild_id()
                .unwrap()
                .get())
            .await {
            Ok(next_words) => next_words,
            Err(_) => return None
        };

        // choose a random next word id
        // make next word become current word and repeat
        current_word_id = match next_words.choose(&mut rand::rng()) {
            Some(word) => word.next_word_id,
            None => break,
        };

    }

    let punct_map = get_punctuation_map();
    for x in 0..generated_text.len() {
        if generated_text[x].len() > 0 {
            continue
        }

        if let None = generated_text.get(x - 1) {
            continue
        }

        if let Some(behavior) = punct_map.get(&generated_text[x].chars().nth(0).unwrap()) {
            if let &Attach::Before = behavior {
                generated_text[x - 1] = generated_text[x - 1].clone() + &*generated_text[x];
                generated_text.remove(x);
            }
        }

        if let None = generated_text.get(x + 1) {
            continue
        }

        if let Some(behavior) = punct_map.get(&generated_text[x].chars().nth(0).unwrap()) {
            if let &Attach::After = behavior {
                generated_text[x + 1] = generated_text[x + 1].clone() + &*generated_text[x];
                generated_text.remove(x);
            }
        }

        if let Some(behavior) = punct_map.get(&generated_text[x].chars().nth(0).unwrap()) {
            if let &Attach::Both = behavior {
                generated_text[x - 1] = generated_text[x - 1].clone() + &*generated_text[x] + &*generated_text[x + 1];
                generated_text.remove(x);
                generated_text.remove(x + 1);
            }
        }
    }
    Some(generated_text.join(" "))
}
