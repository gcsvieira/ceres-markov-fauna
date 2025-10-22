use std::io;
use crate::discord::answers::Answers;
use crate::storage::guild_config_model::Config;
use crate::utils::file_utils::FileOperations;
use crate::core::text_handler::generate_text;

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
pub(super) const GENERATE: &str = "generate";
pub(super) const CHANGE_COMMAND_INDICATOR: &str = "change-com-indicator";
pub(super) const HELLO: &str = "hello_old";

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
    Generate,
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
            GENERATE => Self::Generate,
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
            Self::Generate => Answers::Generate,
            // Make a stat command that will exhibit most words used 
            Self::Hello => Answers::Hello,
            _ => Answers::Unknown,
        }
    }
    
    pub(crate) fn execute_command(&self, content: Option<String>, guild_id: u64) -> Result<String, io::Error> {
        match self { 
            Self::ChangeCommandIndicator => {
                Config::from_file(guild_id)?
                    .change_command_ind(content.clone()
                        .unwrap()
                        .pop()
                        .unwrap())
                    .save_to_file(guild_id).expect("Failed to change the command indicator for the server");
                
                Ok(format!("Alright! The new command indicator for your server will be \"{}\"!", content.unwrap()))
            }
            Self::ChangePrefix => {
                Config::from_file(guild_id)?
                    .change_prefix(content
                        .clone()
                        .unwrap()
                        .to_string())
                    .save_to_file(guild_id).expect("Failed to change the prefix for server: {}");

                Answers::ChangePrefix.output_answer(content, guild_id)
            }
            Self::Generate => Answers::Generate.output_answer(Some(generate_text(guild_id)?), guild_id),
            Self::Echo => Answers::Echo.output_answer(content, guild_id),
            Self::Ping => Answers::Ping.output_answer(None, guild_id),
            Self::Help => Answers::Help.output_answer(None, guild_id),
            Self::Version => Answers::Version.output_answer(None, guild_id),
            Self::Commands => Answers::Commands.output_answer(None, guild_id),
            Self::TableStatus => Answers::TableStatus.output_answer(None, guild_id),
            Self::ResetTable => Answers::ResetTable.output_answer(None, guild_id),
            Self::Hello => Answers::Hello.output_answer(None, guild_id),
            _ => Answers::Unknown.output_answer(None, guild_id)
        }
    }
}