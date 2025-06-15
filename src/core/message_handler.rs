use crate::discord::commands::Commands;
use crate::storage::app_properties_model::PROPERTIES;
use crate::storage::srv_config_model::Config;
use crate::core::text_handler::store_text;
use std::io;

pub(crate) struct MessageHandler {
    pub(crate) content: String,
    pub(crate) guild_id: u64,
}

impl MessageHandler {
    pub(crate) fn new(content: String, guild_id: u64) -> Self {
        Self {
            content,
            guild_id,
        }
    }

    // this is to process the received msg based on the triggers used (command, mention or if the bot should just store the message)
    // we return an io::Error in case there's a file reading/writing problem with any of the operations
    pub(crate) fn process_message(&self) -> Result<Option<String>, io::Error> {
        // if message starts mentioning the bot, we can already process it without having to read the current trigger of the server
        if self.content.starts_with(&PROPERTIES.bot.id) {
            return Ok(self.using_mention()?)
        }
        
        // if not that, we will try to identify a command trigger on the message by reading the command saved on the file
        // we return a file reading Error in case bot failed to read it
        let current_command = match current_trigger(self.guild_id) {
            Ok(trigger) => trigger,
            Err(e) => { return Err(e) }
        };
        
        match self {
            // if message starts with the command trigger of the server
            msg if msg.content
                .starts_with(&current_command) => {
                Ok(self.using_command(current_command)?)
            },
            // if there's neither mentioning nor command on the message, bot should store it
            _ => {
                store_text(self.content.clone(), self.guild_id)?;
                Ok(None)
            }
        }
    }
    
    fn using_command(&self, current_command: String) -> Result<Option<String>, io::Error> {
        let after_prefix = strip_command_prefix(&self.content, current_command);

        let (command, content) = after_prefix.split_once(' ').unwrap_or((after_prefix.as_str(), ""));

        let command = if command.is_empty() { None } else { Some(command.to_string()) };
        let content = if content.is_empty() { None } else { Some(content.to_string()) };

        Ok(
            Some(Commands::parse_to_command(command)
                .execute_command(content, self.guild_id)?, 
            )
        )
    }

    fn using_mention(&self) -> Result<Option<String>, io::Error> {
        let after_prefix = strip_command_prefix(&self.content, PROPERTIES.bot.id.clone());
        
        let (command, content) = after_prefix
            .trim()
            .split_once(' ')
            .unwrap_or((after_prefix.as_str(), ""));

        let command = if command.is_empty() { None } else { Some(command.to_string()) };
        let content = if content.is_empty() { None } else { Some(content.to_string()) };

        Ok(
            Some(
            Commands::parse_to_command(command)
                .execute_command(content, self.guild_id)?
            )
        )
    }
}

fn current_trigger(guild_id: u64) -> Result<String, io::Error> {
    let config = Config::from_file(guild_id)?;

    // Getting prefix and command from the server's configuration file
    Ok(format!("{}{}", config.prefix, config.command_indicator))
}

fn strip_command_prefix(content: &String, prefix: String) -> String {
    content.strip_prefix(&prefix)
        .unwrap()
        .to_string()
}