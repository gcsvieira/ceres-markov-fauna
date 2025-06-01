use std::fmt;

#[derive(Debug)]
pub enum Answers {
    HelpMsg,
    WelcomeMsg,
    ResetTable,
    TableStatus,
    Version,
    Ping,
    Echo,
    Unknown,
}

impl Answers {
    pub fn as_str(&self) -> String {
        match self {
            Self::HelpMsg => "Aah... I don't know how to help you yet...".to_string(),
            Self::WelcomeMsg => "Hello!! I'm happy to join this server!".to_string(),
            Self::ResetTable => "Awh... You don't like my yapping? Are you sure you want to reset everything...?".to_string(),
            Self::TableStatus => "Eh... but looking at it is kinda boring...".to_string(),
            Self::Echo => "Echo!".to_string(),
            Self::Ping => "Pongies!! It worked! ehehe".to_string(),
            Self::Version => format!("We're currently on version {}!", env!("CARGO_PKG_VERSION").to_string()),
            _ => "I... I can't understand that...".to_string(),
        }
    }
}

impl fmt::Display for Answers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}