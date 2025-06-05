use std::io;

pub trait FileOperations {
    fn read_file(guild_id: u64) -> Result<Self, io::Error>
    where
        Self: Sized;

    fn change_prefix(self, prefix: String) -> Self
    where
        Self: Sized;

    fn change_command_ind(self, command: char) -> Self
    where 
        Self: Sized;

    fn save_to_config(&self) -> Result<(), io::Error>
    where 
        Self: Sized;
}