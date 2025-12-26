use std::fmt::format;
use log::info;
use serenity::all::{ButtonStyle, ReactionType};
use poise::{serenity_prelude as serenity, CreateReply};
use serenity::builder::{CreateActionRow, CreateButton};
use serenity::collector::{ComponentInteractionCollector, ReactionCollector};
use serenity::futures::StreamExt;
use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command, default_member_permissions = "ADMINISTRATOR")]
pub(crate) async fn reset_table(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let button_confirm = CreateButton::new("b_confirm")
        .emoji(ReactionType::Unicode("⭕".to_string()))
        .label("Confirm")
        .style(ButtonStyle::Secondary);

    let button_cancel = CreateButton::new("b_cancel")
        .emoji(ReactionType::Unicode("❌".to_string()))
        .label("Cancel")
        .style(ButtonStyle::Secondary);

    let buttons: Vec<CreateButton> = vec![button_confirm, button_cancel];
    let car: Vec<CreateActionRow> = vec![CreateActionRow::Buttons(buttons)];

    let reply = ctx.send(CreateReply::default()
        .reply(true)
        .content("Are you sure you want to delete all the words stored for this server? This can't be undone you know?!")
        .components(car))
        .await?;

    let mut collector = reply.message().await?
        .await_component_interaction(&ctx)
        .timeout(std::time::Duration::from_secs(60))
        .stream();

    if let Some(mci) = collector.next().await {
        let content = match mci.data.custom_id.as_str() {
            "b_confirm" => "Someone confirmed the request!",
            "b_cancel" => "Someone canceled the request!",
            _ => "Unknown selection!"
        };

        mci.create_response(&ctx, serenity::CreateInteractionResponse::UpdateMessage(
            serenity::CreateInteractionResponseMessage::new()
                .content(content)
                .components(vec![])
        ))
            .await?;
    }
    Ok(())
}