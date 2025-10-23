use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, slash_command)]
pub(crate) async fn version(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.reply(format!("We're currently on version {}!", env!("CARGO_PKG_VERSION").to_string())).await?;
    Ok(())
}