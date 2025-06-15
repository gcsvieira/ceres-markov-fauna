use std::error::Error;
use crate::core::message_handler::MessageHandler;
use log::{info, error};
use serenity::all::{ChannelId, CreateMessage, Guild};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::fs;
use std::io;
use crate::discord::answers::Answers;
use crate::storage::srv_config_model::Config;
use crate::storage::srv_markov_model::Markov;
use crate::utils::file_utils::FileOperations;

pub(crate) struct Handler;

#[async_trait]
impl EventHandler for Handler {

    // This will trigger whenever a new server adds the bot or when the bot service is started.
    // In the latter case, it will check all the servers that currently have the bot.
    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {

        let guild_config_exists = match fs::exists(Config::srv_file_path(guild.id.get())) {
            Ok(exists) => exists,
            Err(e) => {error!("Failed to find {}'s srv_config!", guild.name); return}
        };

        let guild_markov_exists = match fs::exists(Markov::srv_file_path(guild.id.get())) {
            Ok(exists) => exists,
            Err(e) => {error!("Failed to find {}'s srv_markov!", guild.name); return}
        };

        info!("Guild {0} was detected:\n  -> {1}\n  -> {2}",
            guild.name,
            if guild_config_exists { "Config files exist" } else { "Config files DON'T exist. Creating them..." },
            if guild_markov_exists { "Markov table exists" } else { "Markov table DOES NOT exist. Creating them..." },
        );

        // If none of the config files exist, create a directory for them first
        if !guild_markov_exists && !guild_config_exists {
            fs::create_dir(format!("srv_storage/{}", guild.id.get()))
                .unwrap_or_else(|e| {
                    error!(
                        "Could not create config directory. {}",
                        e.to_string()
                    );
                });

            // Sending a welcome message, since this server is new
            let welcome_msg = CreateMessage::new()
                .content(Answers::Welcome
                    .output_answer(None, guild.id.get())
                    .unwrap());

            // Sending it to the system channel if it exists
            if let Some(channel_id) = guild.system_channel_id {
                channel_id
                    .send_message(&ctx.http, welcome_msg)
                    .await
                    .expect("Could not send welcome msg.");
            };
        };

        // With the server directory existing, we can now create the individual json files.
        if !guild_config_exists {
            Config::new().save_to_file(guild.id.get()).unwrap_or_else(|e| {
                error!("Could not create standard config file for {0}: {1}", guild.name, e);
            });
        }

        if !guild_markov_exists {
            Markov::new().save_to_file(guild.id.get()).unwrap_or_else(|e| {
                error!("Could not create standard markov file for {0}. {1}", guild.name, e);
            });
        }
    }

    // this will trigger whenever any message is sent on the servers the bot's in
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot { return };
        
        let msg_handler = MessageHandler::new(msg.content, msg.guild_id.unwrap().get());
        
        msg_handler
            .process_message()
            .map_err(|e| {
                error!("There was an error when checking {0}'s message type: {1}", msg.author, e);
                e
            })
            .ok();
        
        let answer = msg_handler
            .process_message()
            .unwrap_or_else(|e| {
            error!("There was an error when processing {0}'s message: {1}", msg.author, e);

            // Attach None to "answer" if there was an error.
            None
        });

        // If "answer" is None, no message will be sent
        if answer.is_some() {
            answer_message(answer.unwrap(), ctx, msg.channel_id).await;
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
