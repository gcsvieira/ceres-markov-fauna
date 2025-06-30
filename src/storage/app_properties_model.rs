use std::fs;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Properties {
    pub(crate) bot: Bot,
    #[serde(rename = "std_config")]
    pub(crate) std_config: StdConfig
}

#[derive(Debug, Deserialize)]
pub(crate) struct Bot {
    pub(crate) id: String
}

#[derive(Debug, Deserialize)]
pub(crate) struct StdConfig {
    pub(crate) prefix: String,
    pub(crate) command_indicator: char,
    pub(crate) lone_word_prob: u8,
    pub(crate) consider_frequency: bool,
}

pub(crate) static PROPERTIES: Lazy<Properties> = Lazy::new(|| {
    load_config_from_file().expect("Could not read properties file")
});

fn load_config_from_file() -> Result<Properties, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string("application.yaml")?;
    let properties: Properties = serde_yaml::from_str(&config_content)?;
    Ok(properties)
}

