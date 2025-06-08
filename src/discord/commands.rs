use std::io;
use crate::discord::answers::Answers;
use crate::storage::srv_config_model::Config;
use crate::utils::file_utils::FileOperations;

pub(super) const HELP: &str = "help";
pub(super) const VERSION: &str = "version";
pub(super) const LONE_WORD_PROB: &str = "lone-word-prob";
pub(super) const CONSIDER_FREQUENCY: &str = "consider-frequency";
pub(super) const TABLE_STATUS: &str = "table-status";
pub(super) const RESET_TABLE: &str = "reset-table";
pub(crate) const ECHO: &str = "echo";
pub(super) const PING: &str = "ping";
pub(super) const CHANGE_PREFIX: &str = "change-prefix";
pub(super) const COMMANDS: &str = "commands";
pub(super) const CHANGE_COMMAND_INDICATOR: &str = "change-com-indicator";
pub(super) const HELLO: &str = "hello";

pub(crate) enum Commands {
    Help,
    Version,
    Commands,
    LoneWordProb,
    ConsiderFrequency,
    TableStatus,
    ResetTable,
    Echo,
    Ping,
    ChangePrefix,
    ChangeCommandIndicator,
    Hello,
    Unknown,
}

impl Commands {

    pub(crate) fn parse_to_command(command: Option<String>) -> Commands {
        let com = command.unwrap_or("unknown".to_string());
        
        match com.as_str() {
            HELP => Self::Help,
            COMMANDS => Self::Commands,
            VERSION => Self::Version,
            LONE_WORD_PROB => Self::LoneWordProb,
            CONSIDER_FREQUENCY => Self::ConsiderFrequency,
            TABLE_STATUS => Self::TableStatus,
            RESET_TABLE => Self::ResetTable,
            ECHO => Self::Echo,
            CHANGE_PREFIX => Self::ChangePrefix,
            CHANGE_COMMAND_INDICATOR => Self::ChangeCommandIndicator,
            PING => Self::Ping,
            s if s.contains(HELLO) => Self::Hello,
            _ => Self::Unknown,
        }
    }

    pub(crate) fn command_to_answer(&self) -> Answers {
        match self {
            Self::Help => Answers::Help,
            Self::Ping => Answers::Ping,
            Self::Commands => Answers::Commands,
            Self::ResetTable => Answers::ResetTable,
            Self::TableStatus => Answers::TableStatus,
            Self::Version => Answers::Version,
            Self::Echo => Answers::Echo,
            Self::ChangePrefix => Answers::ChangePrefix,
            Self::ChangeCommandIndicator => Answers::ChangeCommandIndicator,
            Self::Hello => Answers::Hello,
            _ => Answers::Unknown,
        }
    }
    
    pub(crate) fn execute_command(&self, content: &Option<String>, guild_id: u64) -> Result<&Commands, io::Error> {
        match self { 
            Commands::ChangeCommandIndicator => {
                Config::from_file(guild_id)?
                    .change_command_ind(content.clone()
                        .unwrap()
                        .pop()
                        .unwrap())
                    .save_to_file(guild_id).expect("Failed to change the command indicator for the server");
                
                Ok(&Commands::ChangeCommandIndicator)
            }
            Commands::ChangePrefix => {
                Config::from_file(guild_id)?
                    .change_prefix(content
                        .clone()
                        .unwrap()
                        .to_string())
                    .save_to_file(guild_id).expect("Failed to change the prefix for server: {}");

                Ok(&Commands::ChangePrefix)
            }
            _ => Ok(self)
        }
    }
}