use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command)]
pub(crate) async fn word_count(
    ctx: Context<'_>,
) -> Result<(), Error> {
    match ctx
        .data().db
        .count_words(ctx.guild_id().unwrap().get())
        .await {
        Ok(Some(count)) => ctx.reply(format!("This server currently has {} word(s) registered!", count)),
        Ok(None) => ctx.reply("I couldn't read the amount of words you have here! Maybe you don't have words registered...?".to_string()),
        _ => ctx.reply("An error happened while trying to count the words! Can you contact the one who developed me..?".to_string())
    }.await?;

    Ok(())
}