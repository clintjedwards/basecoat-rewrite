pub mod api;
pub mod cli;
use config::{Config, FileFormat};
use rust_embed::RustEmbed;
use std::{borrow::Cow, error::Error};

#[derive(RustEmbed)]
#[folder = "src/conf/"]
#[include = "*.toml"]
struct EmbeddedConfigFS;

/// The configuration type.
pub enum Kind {
    Api(api::Config),
    Cli(cli::Config),
}

/// returns an embedded default configuration file in bytes.
fn default_config(kind: &Kind) -> Cow<'static, [u8]> {
    match kind {
        Kind::Api(_) => {
            let config_file = EmbeddedConfigFS::get("default_api_config.toml").unwrap();
            config_file.data
        }
        Kind::Cli(_) => {
            let config_file = EmbeddedConfigFS::get("default_cli_config.toml").unwrap();
            config_file.data
        }
    }
}

/// returns a correctly deserialized config struct from the configuration files and environment passed to it.
///
/// The order of the configuration files read in is by order passed in. So [config_1.yml, config_2.yml] would cause
/// any conflicting keys in both configs to inherit config_2's value.
pub fn parse(kind: Kind, paths: Vec<String>) -> Result<Kind, Box<dyn Error>> {
    let mut config_src_builder = Config::builder();

    // First parse embedded config defaults.
    let default_config_raw = default_config(&kind);
    let default_config = std::str::from_utf8(&default_config_raw)?;

    config_src_builder =
        config_src_builder.add_source(config::File::from_str(default_config, FileFormat::Toml));

    // Then parse user given paths.
    for path in paths {
        config_src_builder =
            config_src_builder.add_source(config::File::with_name(&path).required(false));
    }
    let config_src = config_src_builder
        .add_source(config::Environment::with_prefix("BASECOAT").ignore_empty(true))
        .build()?;

    // Then attempt to deserialize based on which config needed.
    match kind {
        Kind::Api(_) => {
            let parsed_config = config_src.try_deserialize::<api::Config>()?;
            Ok(Kind::Api(parsed_config))
        }
        Kind::Cli(_) => {
            let parsed_config = config_src.try_deserialize::<cli::Config>()?;
            Ok(Kind::Cli(parsed_config))
        }
    }
}
