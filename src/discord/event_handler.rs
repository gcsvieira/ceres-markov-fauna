use crate::core::message_handler::MessageHandler;
use serenity::all::{ChannelId, Guild};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::io;

pub(crate) struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {
        // receber informações do servidor novo
        // pegar guild.id
        // criar aquivo config padrão pra esse servidor (configs no yaml)
        // criar tabela com definições padrão
        // enviar mensagem de boas vindas
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let answer =
            process_message(msg.content, msg.guild_id.unwrap().get()).unwrap_or_else(|e| {
                println!("There was an error regarding file io: {}", e);
                None
            });

        if answer.is_some() {
            answer_message(answer.unwrap(), ctx, msg.channel_id).await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn process_message(msg: String, guild_id: u64) -> Result<Option<String>, io::Error> {
    let mut msg_handler: MessageHandler = MessageHandler::new(msg);
    msg_handler.check_msg_type(guild_id)?.parse(guild_id)
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
