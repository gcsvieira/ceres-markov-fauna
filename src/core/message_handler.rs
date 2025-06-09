use std::collections::{HashMap};
use std::io;
use crate::discord::commands::Commands;
use crate::storage::app_properties_model::PROPERTIES;
use crate::storage::srv_config_model::Config;
use crate::storage::srv_markov_model::{Markov, WordEntry};
use crate::utils::file_utils::FileOperations;

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
            msg if msg.content.contains(current_trigger(self.guild_id)?.as_str()) => 't', // -> trigger
            msg if msg.content.contains(&PROPERTIES.bot.id) => 'm', // -> mention
            _ => 's'
        });
        
        Ok(self)
    }

    pub(crate) fn parse(&mut self) -> Result<Option<String>, io::Error> {
        // With the msg_type set, we can just redirect to the corresponding data processing function
        // parse() will receive a String containing the result of the command used. That string will be what the bot will respond to the server
        match self.msg_type {
            Some('t') => Ok(self.using_command()?),
            Some('m') => Ok(self.using_mention()?),
            _ => Ok(self.store_data()?)
        }
    }

    fn using_command(&self) -> Result<Option<String>, io::Error> {
        // We're doing a lot of things here.
        // 1. We create an after_prefix variable
        // 2. A few things will happen before value attribution:
        //      2.1 We'll strip the command trigger from the message's content
        //      2.2 We'll attribute the result of that function to a match
        //      2.3 Since strip_prefix returns an Option<>, we're basically unwrapping the Option<&str> if there is some value in there
        //      2.4 If, after stripping the pattern there's nothing left, then that means 
        //          only the prefix was sent as a message (message was "cf!" instead of "cf!help") so we return None to that Option<>
        //      2.5 In the end, after_prefix will be either a &str or return None
        let after_prefix = match self.content.strip_prefix(current_trigger(self.guild_id)?.as_str()) {
            Some(str) => str,
            None => return Ok(None),
        };
        
        // We create a tuple
        // We split command ("help" from "cf!help) and content using a space
        // We use unwrap to remove it from Option<>
        // If there's no space to use as a splitter, then that means content wasn't sent, so we use a default tuple of (command, empty string).
        let (command, content) = after_prefix
            .split_once(' ')
            .unwrap_or((after_prefix, ""));

        let command = if command.is_empty() { None } else { Some(command.to_string()) };
        let content = if content.is_empty() { None } else { Some(content.to_string()) };

        Ok(Some(Commands::parse_to_command(command)
            .execute_command(&content, self.guild_id)?
            .command_to_answer()
            .output_answer(content, self.guild_id)?))
    }

    fn using_mention(&self) -> Result<Option<String>, io::Error> {
        let after_prefix = match self.content.strip_prefix(&PROPERTIES.bot.id) {
            Some(str) => str.trim(),
            None => return Ok(None),
        };

        // " command content"

        let (command, content) = after_prefix.split_once(' ').unwrap_or((after_prefix, ""));

        let command = if command.is_empty() { None } else { Some(command.to_string()) };
        let content = if content.is_empty() { None } else { Some(content.to_string()) };

        Ok(Some(Commands::parse_to_command(command)
            .execute_command(&content, self.guild_id)?
            .command_to_answer()
            .output_answer(content, self.guild_id)?))
    }

    fn store_data(&self) -> Result<Option<String>, io::Error> {
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