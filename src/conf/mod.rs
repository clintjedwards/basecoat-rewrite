pub mod api;
pub mod cli;
use config::Config;
use std::error::Error;

pub enum Kind {
    Api(api::Config),
    Cli(cli::Config),
}

// get returns a correctly deserialized parsed config from the configuration files and environment passed to it.
// The order of the configuration files read in is by order passed in. So [config_1.yml, config_2.yml] would cause
// any conflicting keys in both configs to inherit config_2's value.
pub fn get(kind: Kind, paths: Vec<String>) -> Result<Kind, Box<dyn Error>> {
    let mut config_src_builder = Config::builder();
    for path in paths {
        config_src_builder =
            config_src_builder.add_source(config::File::with_name(&path).required(false));
    }
    let config_src = config_src_builder
        .add_source(config::Environment::with_prefix("BASECOAT").ignore_empty(true))
        .build()?;

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
