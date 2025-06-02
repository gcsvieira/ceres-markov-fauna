use crate::utils::file_utils::FileOperation;
use crate::discord::commands::Commands;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

pub(crate) struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let pref_com = format!("{}{}", FileOperation::read_command(), FileOperation::read_prefix());
        let after_prefix: (Option<String>, Option<String>, Option<String>);

        if !msg.content.contains(pref_com.as_str()) {
            return;
        }

        after_prefix = parse_message(pref_com, &msg.content);
        let answer= Commands::parse_to_command(&after_prefix.1)
            .execute_command(&after_prefix.2)
            .command_to_answer()
            .send_answer(after_prefix.2);
        send_answer(ctx, msg, answer).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn send_answer(ctx: Context, msg: Message, answer: String) {
    msg.channel_id
        .say(&ctx.http, answer)
        .await
        .expect("Error sending message!");
}

fn parse_message(pref_com: String, msg: &String) -> (Option<String>, Option<String>, Option<String>) {
    let after_prefix = match msg.strip_prefix(pref_com.as_str()) {
        Some(str) => str,
        None => return (None, None, None),
    };

    let (command, content) = after_prefix.split_once(' ').unwrap_or((after_prefix, ""));

    let command = if command.is_empty() { None } else { Some(command.to_string()) };
    let content = if content.is_empty() { None } else { Some(content.to_string()) };

    (Some(pref_com), command, content)
}
