use std::io;
use crate::discord::answers::Answers;
use crate::storage::srv_config_model::Config;
use crate::utils::file_utils::FileOperations;

pub(super) const HELP: &str = "help";
const VERSION: &str = "version";
const LONE_WORD_PROB: &str = "lone-word-prob";
const CONSIDER_FREQUENCY: &str = "consider-frequency";
const TABLE_STATUS: &str = "table-status";
const RESET_TABLE: &str = "reset-table";
const ECHO: &str = "echo";
const PING: &str = "ping";
const CHANGE_PREFIX: &str = "change-prefix";
const CHANGE_COMMAND_INDICATOR: &str = "change-com-indicator";
const HELLO: &str = "hello";

pub(crate) enum Commands {
    Help,
    Version,
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
            Self::ResetTable => Answers::ResetTable,
            Self::TableStatus => Answers::TableStatus,
            Self::Version => Answers::Version,
            Self::ChangePrefix => Answers::ChangePrefix,
            Self::ChangeCommandIndicator => Answers::ChangeCommandIndicator,
            Self::Hello => Answers::Hello,
            _ => Answers::Unknown,
        }
    }
    
    pub(crate) fn execute_command(&self, content: &Option<String>, guild_id: u64) -> Result<&Commands, io::Error> {
        match self { 
            Commands::ChangeCommandIndicator => {
                Config::new(guild_id)?
                    .change_command_ind(content.clone()
                        .unwrap()
                        .pop()
                        .unwrap())
                    .save_to_config().expect("Failed to change the command indicator for the server");


                Ok(&Commands::ChangeCommandIndicator)
            }
            Commands::ChangePrefix => {
                Config::new(guild_id)?
                    .change_prefix(content
                        .clone()
                        .unwrap()
                        .to_string())
                    .save_to_config().expect("Failed to change the prefix for server: {}");

                Ok(&Commands::ChangePrefix)
            }
            Commands::Echo => {
                // TODO: find a way to implement echo
                Ok(&Commands::Echo)
            }
            _ => Ok(self)
        }
    }

    pub(crate) fn describe_command(&self) -> String {
        match self {
            Self::Help => format!("- **{HELP}**: You'll get this message with all the commands you need!"),
            Self::Version => format!("- **{VERSION}**: I will send you my current version."),
            Self::LoneWordProb => format!("- **{LONE_WORD_PROB} <0...100>**: this will change the frequency of lone words on your sentences (words not connected to any sentence)!"),
            Self::ConsiderFrequency => format!("- **{CONSIDER_FREQUENCY} <yes/no>**: when generating sentences, this will take the amount of times a word has appeared in relation to its previous word into consideration!"),
            Self::TableStatus => format!("- **{TABLE_STATUS}**: Current status of your table! How many words and words next to it exists!"),
            Self::ResetTable => format!("- **{RESET_TABLE}**: This will remove all the words stored on this server's table! This is unrecoverable so you be careful, alright?"),
            Self::Echo => format!("- **{ECHO} <msg>**: I will repeat what you say! Don't make me say weird stuff okay?"),
            Self::Ping => format!("- **{PING}**: I will reply you with a pong! Hehehe"),
            Self::ChangePrefix => format!("- **{CHANGE_PREFIX}**: This will change the prefix you use to call me. If your prefix is \"cf!\", then prefix is \"cf\"!"),
            Self::ChangeCommandIndicator => format!("- **{CHANGE_COMMAND_INDICATOR}**: This is the command indicator of your prefix! If your prefix is \"cf!\" then the indicator is \"!\"!"),
            Self::Hello => format!("- **{HELLO}**: I'll talk to you!!"),
            _ => "- It's when you a bit creative with your wording and I couldn't get it!".to_string(),
        }
    }
}