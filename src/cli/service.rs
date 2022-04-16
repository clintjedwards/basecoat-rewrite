use crate::api;
use crate::conf;
use crate::proto::basecoat_client::BasecoatClient;
use crate::proto::GetSystemInfoRequest;
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
    Info,
}

pub async fn start(config: conf::api::Config) {
    api::Api::new(config).await.start().await;
}

pub async fn info(config: conf::cli::Config) -> Result<(), Box<dyn Error>> {
    let channel = tonic::transport::Channel::from_shared(config.server.to_string())?;
    let conn = channel.connect().await?;

    let mut client = BasecoatClient::new(conn);
    let request = tonic::Request::new(GetSystemInfoRequest {});
    let response = client.get_system_info(request).await?.into_inner();
    println!("{:?}", response);
    Ok(())
}
