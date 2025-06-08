use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::from_str;

pub trait FileOperations {
    fn read_file(guild_id: u64) -> Result<Self, io::Error> where Self: Sized + DeserializeOwned
    {
        let raw_file = std::fs::read_to_string(Self::srv_file_path(guild_id))?;
        let file = from_str::<Self>(raw_file.as_str())?;
        Ok(file)
    }

    fn save_to_file(&self, guild_id: u64) -> Result<(), io::Error> where Self: Sized + Serialize {
        let file = File::create(Self::srv_file_path(guild_id))?;
        let mut writer = BufWriter::new(file);

        serde_json::to_writer_pretty(&mut writer, &self)?;
        writer.flush()
    }

    fn srv_file_path(guild_id: u64) -> String;
}