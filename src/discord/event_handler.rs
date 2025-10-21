use crate::core::message_handler::MessageHandler;
use crate::storage::guild_config_model::Config;
use log::{error, info, warn};
use serenity::all::{ChannelId, CreateMessage, Guild};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::fs;
use serenity::futures::future::ok;
use crate::storage::db_client::DbClient;
use crate::discord::answers::Answers;
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

        fs::exists(Config::guild_file_path(guild.id.get()))
            .map_or_else(
                |e| error!("Could not create standard config file for {0}: {1}", &guild_name_uppercase, e),
                |config_file_bool| match config_file_bool {
                    true => info!("[{}] Guild's config file is present.", &guild_name_uppercase),
                    false => {
                        warn!("[{}] Guild's config file DOESN'T exist. Generating it...", &guild_name_uppercase);
                        Config::new()
                            .save_to_file(guild.id.get())
                            .map_or_else(
                                |e| error!("[{}] Could not create standard config file for : {}", &guild_name_uppercase, e),
                                |()| info!("[{}] Guild's config file was created successfully.", &guild_name_uppercase)
                            )
                    }
                }
            );
    }

    // this will trigger whenever any message is sent on the servers the bot's in
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        };

        let msg_handler = MessageHandler::new(msg.content, msg.guild_id.unwrap().get());

        let answer = msg_handler
            .process_message(&self.db)
            .await;

        // If "answer" is None, no message will be sent
        if let Some(answer) = answer {
            answer_message(answer, ctx, msg.channel_id).await;
        }
    }

    // TODO: create sentence generation algorithm

    // This will tell you if the bot's connection to discord api was successful
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

/// Sends a message to a channel on discord.
///
/// Primarily assumes you're answering to a request, which means active calls from the bot might have to be sent through other methods.
async fn answer_message(content: String, ctx: Context, channel_id: ChannelId) {
    channel_id
        .say(&ctx.http, content)
        .await
        .expect("Error sending message!");
}
