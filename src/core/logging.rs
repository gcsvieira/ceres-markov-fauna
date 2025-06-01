enum Logging {
    MsgReadSuccess,
    MsgReadFail,

    StringGenerationSuccess,
    StringGenerationFail,

    MarkovCreationSuccess,
    MarkovCreationFail,

    SentenceDivideSuccess,
    SentenceDivideFail,
    
    Unknown,
}

impl Logging {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "msg_success" => Logging::MsgReadSuccess,
            "msg_fail" => Logging::MsgReadFail,
            "string_gen_success" => Logging::StringGenerationSuccess,
            "string_gen_fail" => Logging::StringGenerationFail,
            "markov_creation_success" => Logging::MarkovCreationSuccess,
            "markov_creation_fail" => Logging::MarkovCreationFail,
            "sentence_divide_success" => Logging::SentenceDivideSuccess,
            "sentence_divide_fail" => Logging::SentenceDivideFail,
            _ => Logging::Unknown,
        }
    }
}