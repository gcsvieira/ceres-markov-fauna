use crate::discord::event_handler;
use serenity::prelude::*;
use dotenv::dotenv;
use std::env;
use log::error;

mod core;
mod discord;
mod markov;
mod storage;
mod utils;

use serenity::Client;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILDS |
        GatewayIntents::GUILD_MESSAGES |
        GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(event_handler::Handler)
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
