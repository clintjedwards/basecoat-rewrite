use serde::Deserialize;

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Config {
    pub general: General,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct General {
    pub debug: bool,
    pub log_level: String,
    pub url: String,
}
