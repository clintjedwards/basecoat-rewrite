use serde::Deserialize;

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Config {
    pub general: General,
    pub server: Server,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct General {
    pub debug: bool,
    pub log_level: String,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Server {
    pub url: String,
    pub storage_path: String,
    pub tls_cert_path: Option<String>,
    pub tls_key_path: Option<String>,
}
