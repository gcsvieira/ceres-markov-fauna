use std::io;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::storage::app_properties_model::PROPERTIES;
use crate::utils::file_utils::FileOperations;

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Config {
    pub(crate) prefix: String,
    pub(crate) command_indicator: char,
    pub(crate) lone_word_prob: u8,
    pub(crate) consider_frequency: bool,
}

impl Config {
    pub(crate) fn new() -> Self {
        Self {
            prefix: PROPERTIES.std_config.prefix.clone(),
            command_indicator: PROPERTIES.std_config.command_indicator,
            lone_word_prob: PROPERTIES.std_config.lone_word_prob,
            consider_frequency: PROPERTIES.std_config.consider_frequency,
        }
    }

    pub(crate) fn from_file(guild_id: u64) -> Result<Self, io::Error> {
        let config = Self::read_file(guild_id)?;

        Ok(Self {
            prefix: config.prefix,
            command_indicator: config.command_indicator,
            lone_word_prob: config.lone_word_prob,
            consider_frequency: config.consider_frequency,
        })
    }

    pub(crate) fn change_prefix(mut self, prefix: String) -> Self {
        self.prefix = prefix;
        json!(self);

        self
    }

    pub(crate) fn change_command_ind(mut self, command: char) -> Self {
        self.command_indicator = command;
        json!(self);

        self
    }
}


impl FileOperations for Config {
    fn guild_file_path(guild_id: u64) -> String {
        format!("db/{}/guild_config.json", guild_id)
    }
}