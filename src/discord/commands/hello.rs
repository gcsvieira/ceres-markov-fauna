use std::io;
use poise::{serenity_prelude as serenity};

pub(crate) struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub(crate) async fn hello(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.say("Hello from poise!!").await?;
    Ok(())
}