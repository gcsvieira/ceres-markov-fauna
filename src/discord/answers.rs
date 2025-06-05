use crate::discord::commands::Commands;
pub(crate) enum Answers {
    Help,
    Welcome,
    ResetTable,
    TableStatus,
    Version,
    Ping,
    Echo,
    ChangePrefix,
    ChangeCommandIndicator,
    Hello,
    Unknown,
}

impl Answers {
    pub(crate) fn output_answer(&self, desired: Option<String>) -> String {
        match self {
            Self::Help => self.help_command(),
            Self::Welcome => "Hello!! I'm happy to join this server! If you have any questions, you can mention me and type help... I hope we can get along!!".to_string(),
            Self::ResetTable => "Awh... You don't like my yapping? Are you sure you want to reset everything...?".to_string(),
            Self::TableStatus => "Eh... but looking at it is kinda boring...".to_string(),
            Self::Ping => "Pongies!! It worked! ehehe".to_string(),
            Self::Version => format!(
                "We're currently on version {}!",
                env!("CARGO_PKG_VERSION").to_string()
            ),
            Self::ChangeCommandIndicator => format!(
                "Alright! The new command indicator for your server will be \"{}\"!",
                desired.unwrap()
            ),
            Self::ChangePrefix => format!(
                "Okay, your new prefix will now be \"{}\"!",
                desired.unwrap()
            ),
            Self::Hello => "Hello!!".to_string(),
            _ => "I... I can't understand that...".to_string(),
        }
    }

    fn help_command(&self) -> String {
        format!(
            "Okay, no worries I'm coming to the rescue! Let me show you what each command does:\n\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            Commands::describe_command(&Commands::Help),
            Commands::describe_command(&Commands::Version),
            Commands::describe_command(&Commands::LoneWordProb),
            Commands::describe_command(&Commands::ConsiderFrequency),
            Commands::describe_command(&Commands::TableStatus),
            Commands::describe_command(&Commands::ResetTable),
            Commands::describe_command(&Commands::Echo),
            Commands::describe_command(&Commands::ChangePrefix),
            Commands::describe_command(&Commands::ChangeCommandIndicator),
            Commands::describe_command(&Commands::Ping),
            Commands::describe_command(&Commands::Hello)
        )
    }
}
