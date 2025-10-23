use std::env::args;
use std::fmt::format;
use log::error;
use serenity::all::CreateMessage;
use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command, track_deletion)]
pub(crate) async fn echo(
    ctx: Context<'_>,
    #[description = "Type your message here!"] arg1: Option<String>,
) -> Result<(), Error> {
    match arg1 {
        Some(arg) => {
            ctx.say(arg).await?;

            if let Context::Prefix(prefix_command) = ctx {
                if let Err(why) = prefix_command.msg.delete(&prefix_command.serenity_context.http).await {
                    ctx.author().direct_message(&prefix_command.serenity_context.http,
                                                CreateMessage::new()
                                                    .content(format!("I was supposed to delete your message in {} but I couldn't! Here's what happened: {}",
                                                                     ctx.guild()
                                                                         .map_or("ERROR_GUILD".to_string(), |guild| guild.name.to_string()),
                                                                     why)
                                                    )
                    ).await?;

                    error!("[{}] Couldn't delete message: {}", ctx.guild().map_or("NO_GUILD".to_string(), |guild| guild.name.to_string()), why);
                }
            }
        }
        None => {
            ctx.say("You didn't type anything!! Are you trying to trick me?!").await?;
        }
    }
    Ok(())
}