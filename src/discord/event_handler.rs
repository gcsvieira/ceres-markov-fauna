use crate::core::message_handler::MessageHandler;
use crate::storage::guild_config_model::Config;
use log::{error, info, warn};
use poise::serenity_prelude as serenity;
use serenity::all::{ChannelId, CreateMessage, Guild};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::fs;
use crate::core::text_handler::store_text;
use crate::storage::db_client::DbClient;
use crate::discord::answers::Answers;
use crate::storage::app_properties_model::PROPERTIES;
use crate::utils::file_utils::FileOperations;

pub(crate) struct Handler {
    pub(crate) db: DbClient
}

#[async_trait]
impl EventHandler for Handler {
    // This will trigger whenever a new server adds the bot or when the bot service is started.
    // In the latter case, it will check all the servers that currently have the bot.
    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {
        let guild_name_uppercase = guild.name.to_ascii_uppercase();

        info!("[{}] Guild detected.", &guild_name_uppercase);

        if let Ok(None) = self.db.is_guild_new(guild.id.get()).await {
            if let Some(channel_id) = guild.system_channel_id {
                let welcome_msg = CreateMessage::new()
                    .content(Answers::Welcome
                        .output_answer(None, guild.id.get())
                        .unwrap());

                channel_id
                    .send_message(&ctx.http, welcome_msg)
                    .await
                    .map_err(|e| error!("[{}] Failed to send welcome message to the system channel: {}", &guild_name_uppercase, e))
                    .ok();

                match self.db.store_guild(guild.id.get(), guild.name, guild.system_channel_id.map(|ch| ch.get()).or(None)).await {
                    Ok(_) => info!("[{}] Guild's information was stored successfully.", &guild_name_uppercase),
                    Err(_) => error!("[{}] Failed to store guild's information.", &guild_name_uppercase),
                };
            }
        }
    }

    // this will trigger whenever any message is sent on the servers the bot's in
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        };

        if msg.content.starts_with(&PROPERTIES.bot.id) {
            return;
        }
        
        if msg.guild_id.is_none() {
            return;
        }

        match store_text(msg.content.to_string(), msg.guild_id.unwrap().get(), &self.db).await {
            Ok(_) => info!("[{}] Stored message successfully.", &msg.guild_id.unwrap().get()),
            Err(e) => error!("[{}] Failed to store message: {}", &msg.guild_id.unwrap(), e),
        }
    }

    // TODO: create sentence generation algorithm

    // This will tell you if the bot's connection to discord api was successful
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}