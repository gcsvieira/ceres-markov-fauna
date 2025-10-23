use serenity::all::{ReactionType};
use poise::serenity_prelude as serenity;
use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command, required_permissions = "ADMINISTRATOR")]
pub(crate) async fn reset_table(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let reply = ctx.say("Are you sure you want to delete all the words stored for this server? This can't be undone you know?!").await?;
    let msg = reply.message().await?;

    msg.react(ctx.http(), ReactionType::Unicode("⭕".to_string())).await?;
    msg.react(ctx.http(), ReactionType::Unicode("❌".to_string())).await?;

    Ok(())
}