use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command)]
pub(crate) async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("Hello!!").await?;
    Ok(())
}

