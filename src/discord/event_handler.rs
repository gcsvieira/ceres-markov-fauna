use serde_json::from_str;
use crate::discord::commands::Commands;
use crate::discord::answers::Answers;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use crate::storage::json_model::Changeable;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        
        let json = std::fs::read_to_string("config.json").unwrap();
        let prefix: char = from_str::<Changeable>(&json)
            .unwrap()
            .prefix.parse()
            .unwrap();

        if msg.content.chars().next().unwrap() == prefix {
            let split_msg: (&char, Commands) = (&msg.content[..1].parse().unwrap(), Commands::from_str(&msg.content[1..]));

            match split_msg.1 {
                Commands::Help => send_answer(ctx, msg, Answers::HelpMsg.as_str()).await,
                Commands::Ping => send_answer(ctx, msg, Answers::Ping.as_str()).await,
                Commands::ResetTable => send_answer(ctx, msg, Answers::ResetTable.as_str()).await,
                Commands::TableStatus => send_answer(ctx, msg, Answers::TableStatus.as_str()).await,
                Commands::Version => send_answer(ctx, msg, Answers::Version.as_str()).await,
                _ => send_answer(ctx, msg, Answers::Unknown.as_str()).await,
            }
        };
        
        async fn send_answer(ctx: Context, msg: Message, answer: String) {
            msg.channel_id.say(&ctx.http, answer).await.expect("Error sending message!");
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}