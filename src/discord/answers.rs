use std::io;
use crate::discord::commands::{HELP, VERSION, COMMANDS, LONE_WORD_PROB, CONSIDER_FREQUENCY, TABLE_STATUS, RESET_TABLE, ECHO, PING, CHANGE_PREFIX, CHANGE_COMMAND_INDICATOR, HELLO};
use crate::storage::srv_config_model::Config;
pub(crate) enum Answers {
    Help,
    Welcome,
    ResetTable,
    TableStatus,
    Commands,
    Version,
    Ping,
    Echo,
    ChangePrefix,
    ChangeCommandIndicator,
    Hello,
    Unknown,
}

impl Answers {
    pub(crate) fn output_answer(&self, desired: Option<String>, guild_id: u64) -> Result<String, io::Error> {
        match self {
            Self::Help => Ok(self.help_command()),
            Self::Welcome => Ok("Hello!! I'm happy to join this server! If you have any questions, you can type \"cf!help\"... I hope we can get along!!".to_string()),
            Self::ResetTable => Ok("Awh... You don't like my yapping? Are you sure you want to reset everything...?".to_string()),
            Self::Commands => self.current_commands(guild_id),
            Self::TableStatus => Ok("Eh... but looking at it is kinda boring...".to_string()),
            Self::Ping => Ok("Pongies!! It worked! ehehe".to_string()),
            Self::Version => Ok(format!(
                "We're currently on version {}!",
                env!("CARGO_PKG_VERSION").to_string()
            )),
            Self::ChangeCommandIndicator => Ok(format!(
                "Alright! The new command indicator for your server will be \"{}\"!",
                desired.unwrap())
            ),
            Self::Echo => Ok(desired.unwrap()),
            Self::ChangePrefix => Ok(format!(
                "Okay, your new prefix will now be \"{}\"!",
                desired.unwrap()
            )),
            Self::Hello => Ok("Hello!!".to_string()),
            _ => Ok("I... I can't understand that...".to_string()),
        }
    }

    fn help_command(&self) -> String {
        format!(
            "Okay, no worries! Let me show you what each command does:\n\n\
            - **{HELP}**: You'll get this message with all the commands you need!\n\
            - **{VERSION}**: I will send you my current version.\n\
            - **{COMMANDS} **: Shows you what are the current configurations of this server!\n\
            - **{LONE_WORD_PROB} <0...100>**: this will change the frequency of lone words on your sentences (words not connected to any sentence)!\n\
            - **{CONSIDER_FREQUENCY} <yes/no>**: when generating sentences, this will take the amount of times a word has appeared in relation to its previous word into consideration!\n\
            - **{TABLE_STATUS}**: Current status of your table! How many words and words next to it exists!\n\
            - **{RESET_TABLE}**: This will remove all the words stored on this server's table! This is unrecoverable so you be careful, alright?\n\
            - **{ECHO} <msg>**: I will repeat what you say! Don't make me say weird stuff okay?\n\
            - **{PING}**: I will reply you with a pong! Hehehe\n\
            - **{CHANGE_PREFIX}**: This will change the prefix you use to call me. If your prefix is \"cf!\", then prefix is \"cf\"!\n\
            - **{CHANGE_COMMAND_INDICATOR}**: This is the command indicator of your prefix! If your prefix is \"cf!\" then the indicator is \"!\"!\n\
            - **{HELLO}**: I'll talk to you!!",
        )
    }
    
    fn current_commands(&self, guild_id: u64) -> Result<String, io::Error> {
        let config = Config::from_file(guild_id)?;
        
        Ok(format!(
            "Sure! Here are the current commands of your server:\n\
            - Your current command indicator is \"{0}\"!\n\
            - Your current prefix is \"{1}\"!\n\
            - You are {2}considering frequency to generate sentences!\n\
            - Your lone word use probability is currently set to {3}%!", 
            config.command_indicator, config.prefix, if config.consider_frequency == false { "NOT " } else { "" }, config.lone_word_prob))
    }
}
