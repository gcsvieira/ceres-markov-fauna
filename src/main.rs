use crate::discord::event_handler;
use serenity::prelude::*;
use dotenv::dotenv;
use std::env;
use log::{error, info};

mod core;
mod discord;
mod markov;
mod storage;
mod utils;
mod errors;
pub(crate) mod commands;

use serenity::Client;
use storage::db_client::DbClient;
use commands::hello::hello;
use commands::help::help;
use commands::echo::echo;
use commands::version::version;
use storage::app_properties_model::PROPERTIES;

pub(crate) struct Data;

#[tokio::main]
async fn main() {
    dotenv()
        .ok()
        .map_or_else(|| {error!("Failed to load dotenv. Is it located in the root folder?"); panic!()},
                     |dotenv| {info!("Dotenv file loaded successfully.")});

    env_logger::init();

    let token = match env::var("DISCORD_TOKEN") {
        Ok(token) => {info!("Discord token loaded successfully."); token},
        Err(e) => {error!("There was a problem with the discord token. Check the dotenv file: {}", e); panic!()}
    };

    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let db_client = DbClient::new(&PROPERTIES.db.path)
        .await
        .map_or_else(
        |e| { error!("Problem when creating the client for the db: {}", e); panic!()},
        |con| { info!("Database was connected successfully."); con }
    );

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                hello(),
                help(),
                echo(),
                version()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(event_handler::Handler { db: db_client.clone() })
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);


    }

    /*    let texts = [
        String::from("the mayor bent the fucking knee because the con actually drops a nuclear bomb on the tourism and infrastructure of the city"),
        String::from("at least not in someone's pocket which is nice"),
        String::from("frfr ong"),
        String::from("She's helping at the con"),
        String::from("Well I got invited to drink some booze with her after the con"),
        String::from("what if i just raped u in public instead"),
        String::from("It's a lease car so no fuck off eat shit and fuck in the mens toilet"),
        String::from("cool let me borrow his car to bone"),
        String::from("I'm actually driving to my friend where we'll sleep and HE will be doing the driving"),
        String::from("he's fucking the shit out of that minor"),
        String::from("Speaking of Poot, if I were to bone at dokomi would you lend me your car"),
        String::from("I have enough people who are willing to do that"),
    ];
    let mut words: Vec<&str> = Vec::new();

    for text in texts.iter() {
        words.extend(text.split_whitespace());
    }

    if words.len() < 2 {
        println!("Sentence is too short to find word pairs.");
        return;
    }

    for i in 0..(words.len() - 1) {
        let current_word = String::from(words[i]);
        let next_word = String::from(words[i + 1]);

        let next_word_counts = markov_table
            .entry(current_word)
            .or_insert_with(HashMap::new);

        *next_word_counts.entry(next_word).or_insert(0) += 1;
    }

    for (current_word, next_word) in &markov_table {
        println!("\"{}\"", current_word);
        for (next_word, count) in next_word {
            println!(" -> \"{}\", {}", next_word, count);
        }
    }*/
}
