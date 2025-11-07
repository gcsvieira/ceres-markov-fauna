use rand::Rng;
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
    if quantity.is_some() && quantity.unwrap() > 10 {
        ctx.say("I can only generate up to 10 sentences at once!")
            .await?;
        return Ok(());
    }

    if let Some(0) = quantity {
        ctx.say("Very funny!! Are you playing with me? I won't generate anything then! Hmpf!")
            .await?;
        return Ok(())
    }

    match quantity {
        None | Some(1) => {
            let sentence = generate_sentence(&ctx).await?;
            ctx.say(sentence).await?;
            Ok(())
        },
        Some(quantity) => {
            for _ in 0..quantity {
                let sentence = generate_sentence(&ctx).await?;
                ctx.say(sentence).await?;
            }
            Ok(())
        }
    }
}

async fn generate_sentence(ctx: &Context<'_>) -> Result<String, Error> {
    let mut rng = rand::thread_rng();
    let mut generated_text: Vec<String> = Vec::new();

    let extracted_words: Vec<WordChaining> = ctx.data().db.extract_words(ctx.guild_id().unwrap().get()).await?;

    let sentence_length: u8 = rng.gen_range(6..30);
    let word_random = rng.gen_range(0..extracted_words.len());
    let word_chaining = &extracted_words[word_random];
    let mut current_word_id = Some(word_chaining.word_id);
    let current_next_word_id = Some(word_chaining.next_word_id);

    // TODO: word extraction has to be inside the loop as well. when current word and current next word are set, current next becomes current word and there needs to be another search on the db again
    for _ in 0..sentence_length {
        match ctx.data().db.get_word_pretty(&current_word_id.unwrap()).await? {
            Some(word_pretty) => {
                generated_text.push(word_pretty);
            }
            None => continue
        };

        if let Some(next_word_pretty) = ctx.data().db.get_word_pretty(&current_next_word_id.unwrap()).await? {
            generated_text.push(next_word_pretty);
        }

        current_word_id = current_next_word_id
    }

    Ok(generated_text.join(" "))
}
