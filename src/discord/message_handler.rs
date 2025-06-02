use crate::discord::commands::Commands;
use crate::discord::answers::Answers;

const HELP: &str = "help";
const VERSION: &str = "version";
const LONE_WORD_PROB: &str = "lone-word-prob";
const CONSIDER_FREQUENCY: &str = "consider-frequency";
const TABLE_STATUS: &str = "table-status";
const RESET_TABLE: &str = "reset-table";
const ECHO: &str = "echo";
const PING: &str = "ping";
const UNKNOWN: &str = "unknown";

pub(crate) struct MessageHandler;

impl MessageHandler {
    pub(crate) fn send_answer(msg: &Answers) -> String {
        match msg {
            Answers::Help => "Aah... I don't know how to help you yet...".to_string(),
            Answers::Welcome => "Hello!! I'm happy to join this server!".to_string(),
            Answers::ResetTable => "Awh... You don't like my yapping? Are you sure you want to reset everything...?".to_string(),
            Answers::TableStatus => "Eh... but looking at it is kinda boring...".to_string(),
            Answers::Echo => "Echo!".to_string(),
            Answers::Ping => "Pongies!! It worked! ehehe".to_string(),
            Answers::Version => format!("We're currently on version {}!", env!("CARGO_PKG_VERSION").to_string()),
            _ => "I... I can't understand that...".to_string(),
        }
    }

    pub(crate) fn parse_command(msg: &str) -> &Commands {
        match msg {
            HELP => &Commands::Help,
            VERSION => &Commands::Version,
            LONE_WORD_PROB => &Commands::LoneWordProb,
            CONSIDER_FREQUENCY => &Commands::ConsiderFrequency,
            TABLE_STATUS => &Commands::TableStatus,
            RESET_TABLE => &Commands::ResetTable,
            ECHO => &Commands::Echo,
            PING => &Commands::Ping,
            _ => &Commands::Unknown,
        }
    }

    pub(crate) fn output_command(com: &Commands) -> String {
        match com {
            Commands::Help => HELP.to_string(),
            Commands::Version => VERSION.to_string(),
            Commands::LoneWordProb => LONE_WORD_PROB.to_string(),
            Commands::ConsiderFrequency => CONSIDER_FREQUENCY.to_string(),
            Commands::TableStatus => TABLE_STATUS.to_string(),
            Commands::ResetTable => RESET_TABLE.to_string(),
            Commands::Echo => ECHO.to_string(),
            Commands::Ping => PING.to_string(),
            _ => UNKNOWN.to_string(),
        }
    }}