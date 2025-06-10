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
    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {
        let guild_config_exists = match fs::exists(format!("srv_storage/{}/srv_config.json", guild.id)) {
            Ok(exists) => exists,
            Err(e) => {error!("Failed to find {}'s srv_config!", guild.name); return}
        };
        let guild_markov_exists = match fs::exists(format!("srv_storage/{}/srv_markov.json", guild.id)) {
            Ok(exists) => exists,
            Err(e) => {error!("Failed to find {}'s srv_markov!", guild.name); return}
        };

        info!("Guild {0} was detected:\n  -> {1}\n  -> {2}",
            guild.name,
            if guild_config_exists { "Config files exist" } else { "Config files DON'T exist. Creating them..." },
            if guild_markov_exists { "Markov table exists" } else { "Markov table DOES NOT exist. Creating them..." },
        );

        if !guild_markov_exists && !guild_config_exists {
            fs::create_dir(format!("srv_storage/{}", guild.id.get())).unwrap_or_else(|e| {
                error!(
                    "Could not create config directory. {}",
                    e.to_string()
                );
            });

            let welcome_msg = CreateMessage::new()
                .content(Answers::Welcome
                    .output_answer(None, guild.id.get())
                    .unwrap_or("Could not get welcome msg!".to_string())
                );
            guild.system_channel_id.unwrap().send_message(&ctx.http, welcome_msg).await.expect("Could not send welcome msg.");
        };

        if !guild_config_exists {
            Config::new().save_to_file(guild.id.get()).unwrap_or_else(|e| {
                error!("Could not create standard config file. {}", e);
            });
        }

        if !guild_markov_exists {
            Markov::new().save_to_file(guild.id.get()).unwrap_or_else(|e| {
                error!("Could not create standard markov file. {}", e);
            });
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        // Won't do anything if the message is from a bot.
        if msg.author.bot { return }
        
        // I need two kinds of message processing:
        //  -> I need a string in case I need to answer
        //        The current structure satisfies that.
        //  -> I need the command processing to happen
        //        The current structure struggles with this because even though I have a place to process
        //        commands I don't have something to send the message back in case that processing involved
        //        a string.
        //  
        //   -> The solution is to make process command return an optional string while at the same time
        let answer =
            process_message(msg.content, msg.guild_id.unwrap().get()).unwrap_or_else(|e| {
                error!("There was an error regarding file io: {}", e);
                None
            });

        if answer.is_some() {
            answer_message(answer.unwrap(), ctx, msg.channel_id).await;
        }
    }
    
    // TODO: create sentence generation algorithm

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

fn process_message(msg: String, guild_id: u64) -> Result<Option<String>, io::Error> {    
    let mut msg_handler: MessageHandler = MessageHandler::new(msg, guild_id);
    msg_handler.check_msg_type()?.parse()
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
