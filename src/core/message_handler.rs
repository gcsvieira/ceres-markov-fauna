use crate::core::text_handler::store_text;
use crate::discord::commands::Commands;
use crate::storage::app_properties_model::PROPERTIES;
use crate::storage::guild_config_model::Config;
use log::error;
use std::io;
use std::thread::current;

pub(crate) struct MessageHandler {
    pub(crate) content: String,
    pub(crate) guild_id: u64,
}

impl MessageHandler {
    pub(crate) fn new(content: String, guild_id: u64) -> Self {
        Self { content, guild_id }
    }

    // this is to process the received msg based on the triggers used (command, mention or if the bot should just store the message)
    // we return an io::Error in case there's a file reading/writing problem with any of the operations
    pub(crate) fn process_message(&self) -> Option<String> {
        if self.content.starts_with(&PROPERTIES.bot.id) {
            return self.with_mention()
                .map_or_else(
                    |e| { error!("There was an error trying to process message using mention: {}", e); None },
                    |result| Some(result)
                )
        };
        
        match current_trigger(self.guild_id) {
            Ok(trigger) => {
                if self.content.starts_with(&trigger) {
                    self.with_command(trigger)
                        .inspect_err(|e| { error!("There was an error trying to extract the current command of the guild: {}", e) })
                        .ok()
                } else {
                    store_text(self.content.clone(), self.guild_id)
                        .inspect_err(|e| { error!("There was an error trying to store the message: {}", e) })
                        .ok();

                    None
                }
            },
            Err(e) => { error!("There was an error when trying to obtain guild's current trigger: {}", e); None }
        }
    }

    fn with_command(&self, current_command: String) -> Result<String, io::Error> {
        let after_prefix = strip_command_prefix(&self.content, current_command);

        let (command, content) = after_prefix
            .split_once(' ')
            .unwrap_or((after_prefix.as_str(), ""));

        let command = if command.is_empty() { None } else { Some(command.to_string()) };
        let content = if content.is_empty() { None } else { Some(content.to_string()) };

        Ok(Commands::parse_to_command(command).execute_command(content, self.guild_id)?)
    }

    fn with_mention(&self) -> Result<String, io::Error> {
        let after_prefix = strip_command_prefix(&self.content, PROPERTIES.bot.id.clone());

        let (command, content) = after_prefix
            .trim()
            .split_once(' ')
            .unwrap_or((after_prefix.as_str(), ""));

        let command = if command.is_empty() { None } else { Some(command.to_string()) };
        let content = if content.is_empty() { None } else { Some(content.to_string()) };

        Ok(Commands::parse_to_command(command).execute_command(content, self.guild_id)?)
    }
}

fn current_trigger(guild_id: u64) -> Result<String, io::Error> {
    let config = Config::from_file(guild_id)?;

    // Getting prefix and command from the server's configuration file
    Ok(format!("{}{}", config.prefix, config.command_indicator))
}

fn strip_command_prefix(content: &String, prefix: String) -> String {
    content.strip_prefix(&prefix).unwrap().to_string()
}
