use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
use crate::utils::file_utils::FileOperations;

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Config {
    pub(crate) server_id: u64,
    pub(crate) prefix: String,
    pub(crate) command_indicator: char,
    pub(crate) lone_word_prob: u8,
    pub(crate) consider_frequency: bool,
}

impl Config {
    pub(crate) fn new(guild_id: u64) -> Result<Self, io::Error> {
        let config = Self::read_file(guild_id)?;

        Ok(Self {
            server_id: config.server_id,
            prefix: config.prefix,
            command_indicator: config.command_indicator,
            lone_word_prob: config.lone_word_prob,
            consider_frequency: config.consider_frequency,
        })
    }
}

impl FileOperations for Config {
    fn read_file(guild_id: u64) -> Result<Config, io::Error> {
        let raw_config = std::fs::read_to_string(srv_config_path(guild_id))?;
        let config = from_str::<Config>(raw_config.as_str())?;
        Ok(config)
    }

    fn change_prefix(mut self, prefix: String) -> Self {
        self.prefix = prefix;
        json!(self);

        self
    }

    fn change_command_ind(mut self, command: char) -> Self {
        self.command_indicator = command;
        json!(self);

        self
    }

    fn save_to_config(&self) -> Result<(), io::Error> {
        let file = File::create(srv_config_path(self.server_id))?;
        let mut writer = BufWriter::new(file);
        
        serde_json::to_writer_pretty(&mut writer, &self)?;
        writer.flush()
    }
}

fn srv_config_path(guild_id: u64) -> String {
    format!("srv_configs/{}/srv_config.json", guild_id)
}