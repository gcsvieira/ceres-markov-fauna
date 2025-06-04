use crate::discord::answers::Answers;
use crate::utils::file_utils::FileOperation;

const HELP: &str = "help";
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
const UNKNOWN: &str = "unknown";

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
        let com = command.unwrap_or(UNKNOWN.to_string());
        
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

    pub(crate) fn parse_to_str(&self) -> String {
        match self {
            Self::Help => HELP.to_string(),
            Self::Version => VERSION.to_string(),
            Self::LoneWordProb => LONE_WORD_PROB.to_string(),
            Self::ConsiderFrequency => CONSIDER_FREQUENCY.to_string(),
            Self::TableStatus => TABLE_STATUS.to_string(),
            Self::ResetTable => RESET_TABLE.to_string(),
            Self::Echo => ECHO.to_string(),
            Self::Ping => PING.to_string(),
            Self::ChangePrefix => CHANGE_PREFIX.to_string(),
            Self::ChangeCommandIndicator => CHANGE_COMMAND_INDICATOR.to_string(),
            Self::Hello => HELLO.to_string(),
            _ => UNKNOWN.to_string(),
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
    
    pub(crate) fn execute_command(&self, desired: &Option<String>) -> &Commands {
        match self { 
            Commands::ChangeCommandIndicator => {
                FileOperation::change_command_ind(desired
                    .clone()
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap());
                &Commands::ChangeCommandIndicator
            }
            Commands::ChangePrefix => {
                FileOperation::change_prefix(desired.clone()
                    .unwrap()
                    .to_string());
                &Commands::ChangePrefix
            }
            _ => self
        }
    }
}