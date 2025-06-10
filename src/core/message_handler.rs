use crate::discord::commands::Commands;
use crate::storage::app_properties_model::PROPERTIES;
use crate::storage::srv_config_model::Config;
use crate::storage::srv_markov_model::Markov;
use crate::utils::file_utils::FileOperations;
use std::io;

pub(crate) struct MessageHandler {
    content: String,
    msg_type: Option<char>,
    guild_id: u64,
}

impl MessageHandler {
    pub(crate) fn new(content: String, guild_id: u64) -> Self {
        Self {
            content,
            msg_type: None,
            guild_id,
        }
    }

    /// We modify the mutable [`msg_type`] variable to receive a char corresponding to its message type.
    ///
    /// - It will receive 't' if the message contains the configured trigger of the server
    /// - 'm' if the message is mentioning the bot (for the future interaction system)
    /// - 's' stands for 'store'. We will basically store anything that isn't the 2 types above
    ///
    /// [`msg_type`]: MessageHandler
    pub(crate) fn check_msg_type(&mut self) -> Result<&mut Self, io::Error> {
        self.msg_type = Some(match &self {
            msg if msg.content.contains(current_trigger(self.guild_id)?.as_str()) => { 't' } // -> trigger
            msg if msg.content.contains(&PROPERTIES.bot.id) => 'm', // -> mention
            _ => 's',
        });

        Ok(self)
    }

    pub(crate) fn parse(&mut self) -> Result<Option<String>, io::Error> {
        // With the msg_type set, we can just redirect to the corresponding data processing function
        // parse() will receive a String containing the result of the command used. That string will be what the bot will respond to the server
        match self.msg_type {
            Some('t') => Ok(self.using_command()?),
            Some('m') => Ok(self.using_mention()?),
            _ => Ok(self.store_data()?),
        }
    }

    fn using_command(&self) -> Result<Option<String>, io::Error> {
        let after_prefix = strip_command_prefix(&self.content, current_trigger(self.guild_id)?);

        let (command, content) = after_prefix.split_once(' ').unwrap_or((after_prefix.as_str(), ""));

        let command = if command.is_empty() { None } else { Some(command.to_string()) };
        let content = if content.is_empty() { None } else { Some(content.to_string()) };

        Ok(Some(
            Commands::parse_to_command(command)
                .execute_command(content, self.guild_id)?,
        ))
    }

    fn using_mention(&self) -> Result<Option<String>, io::Error> {
        let after_prefix = match self.content.strip_prefix(&PROPERTIES.bot.id) {
            Some(str) => str.trim(),
            None => return Ok(None),
        };

        let (command, content) = after_prefix.split_once(' ').unwrap_or((after_prefix, ""));

        let command = if command.is_empty() { None } else { Some(command.to_string()) };
        let content = if content.is_empty() { None } else { Some(content.to_string()) };

        Ok(Some(
            Commands::parse_to_command(command)
                .execute_command(content, self.guild_id)?
        ))
    }

    fn store_data(&self) -> Result<Option<String>, io::Error> {
        // TODO: make her randomly send a generated sentence here, say... 5% of chance

        let mut markov = match Markov::from_file(self.guild_id) {
            Ok(markov) => markov,
            Err(e) => return Err(e),
        };

        if self.content.is_empty() {
            return Ok(None);
        }

        let words: Vec<String> = self.content
            .split_whitespace()
            .map(String::from)
            .collect();

        if words.len() < 2 {
            // TODO: check for links, duplicated words and emotes she can't use. They shouldn't be added.
            markov.add_lone_word(words);
        } else {
            for i in 0..(words.len() - 1) {
                let current_word = words[i].clone();
                let next_word = words[i + 1].clone();

                markov.add_word_pair(current_word, next_word);
            }
        }

        markov.save_to_file(self.guild_id)?;
        Ok(None)
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