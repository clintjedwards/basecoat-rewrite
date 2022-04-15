use crate::api;
use crate::conf;
use clap::{Args, Subcommand};
use std::error::Error;

#[derive(Debug, Args)]
pub struct ServiceSubcommands {
    #[clap(subcommand)]
    pub command: ServiceCommands,
}

#[derive(Debug, Subcommand)]
pub enum ServiceCommands {
    /// Start the Basecoat GRPC service.
    #[clap(
        long_about = "Basecoat runs a a GRPC backend combined with GRPC-WEB/HTTP1.
    Running this command attempts to start the long running service. This command will block and only
    gracefully stop on SIGINT or SIGTERM signals."
    )]
    Start,
}

pub fn start(user_config: Option<String>) -> Result<impl std::future::Future, Box<dyn Error>> {
    let config_paths;
    if let Some(user_config) = user_config {
        config_paths = vec![user_config];
    } else {
        config_paths = conf::api::default_config_paths();
    }

    let parsed_config = conf::parse(conf::Kind::Api(conf::api::Config::default()), config_paths)?;

    if let conf::Kind::Api(api_config) = parsed_config {
        Ok(api::Api::new(api_config).start())
    } else {
        Err("could not retrieve correct configuration type".into())
    }
}
