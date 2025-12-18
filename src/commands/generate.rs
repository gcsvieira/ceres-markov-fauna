use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;
use rand::Rng;
use rand::prelude::SliceRandom;
use crate::Data;
use crate::storage::db_model::WordChaining;

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
                let sentence = generate_sentence(&ctx).await;
                sentences = format!("{}\n{}", sentences, sentence)
            }

            if !sentences.is_empty() { ctx.say(sentences).await?; }
            Ok(())
        }
    }
}

async fn generate_sentence(ctx: &Context<'_>) -> Option<String> {
    let sentence_length: u8 = rand::thread_rng().gen_range(6..30);
    let mut generated_text: Vec<String> = Vec::new();

    let extracted_words: Vec<WordChaining> = match ctx.data().db
        .extract_words(ctx.guild_id().unwrap().get())
        .await {
        Ok(words) => words,
        Err(e) => return None
    };

    let mut current_word_id = match extracted_words.choose(&mut rand::thread_rng()) {
        Some(word) => word.word_id,
        None => return None,
    };

    for _ in 0..sentence_length {
        match ctx.data().db.get_word_pretty(&current_word_id).await {
            Ok(word_pretty) => {
                if let Some(w_c) = word_pretty {
                    generated_text.push(w_c);
                }
            }
            Err(_) => continue
        };

        let next_words = match ctx.data().db
            .get_next_words(&current_word_id, ctx.guild_id().unwrap().get()).await {
            Ok(next_words) => next_words,
            Err(_) => return None
        };

        let mut candidates: Vec<u64> = Vec::new();
        let mut weights: Vec<u32> = Vec::new();

        for word in next_words {
            candidates.push(word.word_id);
            weights.push(word.frequency);
        }

        if candidates.is_empty() {
            break;
        }

        let distribution = match WeightedIndex::new(&weights) {
            Ok(dist) => dist,
            Err(_) => {
                break;
            }
        };

        let current_next_word_id: u64 = distribution
            .sample(&mut rand::thread_rng()) as u64;

        current_word_id = current_next_word_id;
    }
    Some(generated_text.join(" "))
}
