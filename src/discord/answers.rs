
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
            Self::Help => "Aah... I don't know how to help you yet...".to_string(),
            Self::Welcome => "Hello!! I'm happy to join this server!".to_string(),
            Self::ResetTable => "Awh... You don't like my yapping? Are you sure you want to reset everything...?".to_string(),
            Self::TableStatus => "Eh... but looking at it is kinda boring...".to_string(),
            Self::Echo => "Echo!".to_string(),
            Self::Ping => "Pongies!! It worked! ehehe".to_string(),
            Self::Version => format!("We're currently on version {}!", env!("CARGO_PKG_VERSION").to_string()),
            Self::ChangeCommandIndicator => format!("Alright! The new command indicator for your server will be \"{}\"!", desired.unwrap()),
            Self::ChangePrefix => format!("Okay, your new prefix will now be \"{}\"!", desired.unwrap()),
            Self::Hello => "Hello!!".to_string(),
            _ => "I... I can't understand that...".to_string(),
        }
    }
}