use crate::discord::event_handler;
use dotenv::dotenv;
use log::{error, info};
use serenity::prelude::*;
use std::env;

mod commands;
mod core;
mod discord;
mod errors;
mod markov;
mod storage;
mod utils;

use crate::commands::reset_table::reset_table;
use errors::framework_error::on_error;
use commands::echo::echo;
use commands::generate::generate;
use commands::hello::hello;
use commands::help::help;
use commands::version::version;
use commands::word_count::word_count;
use core::db_client::DbClient;
use serenity::Client;
use storage::app_properties_model::PROPERTIES;

pub(crate) struct Data {
    pub(crate) db: DbClient,
}

#[tokio::main]
async fn main() {
    dotenv().ok().map_or_else(
        || {
            error!("Failed to load dotenv. Is it located in the root folder?");
            panic!()
        },
        |_dotenv| info!("Dotenv file loaded successfully."),
    );

    env_logger::init();

    let token = match env::var("DISCORD_TOKEN") {
        Ok(token) => {
            info!("Discord token loaded successfully.");
            token
        }
        Err(e) => {
            error!(
                "There was a problem with the discord token. Check the dotenv file: {}",
                e
            );
            panic!()
        }
    };

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let db_client = DbClient::new(&PROPERTIES.db.path).await.map_or_else(
        |e| {
            error!("Problem when creating the client for the db: {}", e);
            panic!()
        },
        |con| {
            info!("Database was connected successfully.");
            con
        },
    );

    let data_db = db_client.clone();
    let client_db = db_client.clone();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                hello(),
                help(),
                echo(),
                version(),
                reset_table(),
                word_count(),
                generate(),
            ],
            // on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { db: data_db })
            })
        })
        .build();

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(event_handler::Handler { db: client_db })
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}