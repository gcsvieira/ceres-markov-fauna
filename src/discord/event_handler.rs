use crate::core::message_handler::MessageHandler;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serenity::all::ChannelId;

pub(crate) struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let answer = process_message(msg.content);
        
        if answer.is_some() {
            send_message(answer.unwrap(), ctx, msg.channel_id).await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn process_message(msg: String) -> Option<String> {
    let mut msg_handler: MessageHandler = MessageHandler::new(msg);
    
    msg_handler
        .check_msg_type()
        .parse()
}

/// Sends a message to a channel on discord.
/// 
/// Primarily assumes you're answering to a request, which means active calls from the bot might have to be sent through other methods.
async fn send_message(content: String, ctx: Context, channel_id: ChannelId) {
    channel_id
        .say(&ctx.http, content)
        .await
        .expect("Error sending message!");
}