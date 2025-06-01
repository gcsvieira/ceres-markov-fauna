use std::fmt::{Display, Formatter, Result as FmtResult};
use serde_json::from_str;
use crate::storage::json_model::Changeable;

const HELP: &str = "help";
const VERSION: &str = "version";
const LONE_WORD_PROB: &str = "lone-word-prob";
const CONSIDER_FREQUENCY: &str = "consider-frequency";
const TABLE_STATUS: &str = "table-status";
const RESET_TABLE: &str = "reset-table";
const ECHO: &str = "echo";
const PING: &str = "ping";
const UNKNOWN: &str = "unknown";

pub enum Commands {
    Help,
    Version,
    LoneWordProb,
    ConsiderFrequency,
    TableStatus,
    ResetTable,
    Echo,
    Ping,
    Unknown,
}

impl Commands {
    pub fn as_str(&self) -> String {
        match self {
            Self::Help => HELP.to_string(),
            Self::Version => VERSION.to_string(),
            Self::LoneWordProb => LONE_WORD_PROB.to_string(),
            Self::ConsiderFrequency => CONSIDER_FREQUENCY.to_string(),
            Self::TableStatus => TABLE_STATUS.to_string(),
            Self::ResetTable => RESET_TABLE.to_string(),
            Self::Echo => ECHO.to_string(),
            Self::Ping => PING.to_string(),
            _ => UNKNOWN.to_string(),
        }
    }

    pub fn from_str(msg: &str) -> Self {
        match msg {
            HELP => Self::Help,
            VERSION => Self::Version,
            LONE_WORD_PROB => Self::LoneWordProb,
            CONSIDER_FREQUENCY => Self::ConsiderFrequency,
            TABLE_STATUS => Self::TableStatus,
            RESET_TABLE => Self::ResetTable,
            ECHO => Self::Echo,
            PING => Self::Ping,
            _ => Self::Unknown,
        }
    }

    pub fn concat_prefix(&self) -> String {
        let json = std::fs::read_to_string("../../config.json").unwrap();
        let changeables = from_str::<Changeable>(&json).unwrap();

        format!("{}{}", changeables.prefix, self.as_str())
    }
}

impl Display for Commands {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.concat_prefix())
    }
}