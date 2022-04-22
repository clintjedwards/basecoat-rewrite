use crate::conf;
use crate::proto::basecoat_client::BasecoatClient;
use crate::proto::{
    CreateColorantRequest, DeleteColorantRequest, DescribeColorantRequest, ListColorantsRequest,
};
use clap::{Args, Subcommand};
use std::error::Error;

#[derive(Debug, Args)]
pub struct ColorantSubcommands {
    #[clap(subcommand)]
    pub command: ColorantCommands,
}

#[derive(Debug, Subcommand)]
pub enum ColorantCommands {
    /// List all colorants.
    List {
        /// Organization ID of target organization.
        org_id: String,
    },

    /// Create a new colorant.
    Create {
        /// Organization ID of target organization.
        org_id: String,
        /// Full name of colorant.
        name: String,
    },

    /// Get details about a specific colorant.
    Describe {
        /// Organization ID of target organization.
        org_id: String,
        /// ID of colorant.
        id: String,
    },

    /// Delete a specific colorant.
    Delete { org_id: String, id: String },
}

pub async fn list(config: conf::cli::Config, org_id: &str) -> Result<(), Box<dyn Error>> {
    let channel = tonic::transport::Channel::from_shared(config.server.to_string())?;
    let conn = channel.connect().await?;

    let mut client = BasecoatClient::new(conn);
    let request = tonic::Request::new(ListColorantsRequest {
        org_id: org_id.to_string(),
    });
    let response = client.list_colorants(request).await?;
    println!("{:?}", response.into_inner().colorants);
    Ok(())
}

pub async fn create(
    config: conf::cli::Config,
    org_id: &str,
    name: &str,
) -> Result<(), Box<dyn Error>> {
    let channel = tonic::transport::Channel::from_shared(config.server.to_string())?;
    let conn = channel.connect().await?;

    let mut client = BasecoatClient::new(conn);
    let request = tonic::Request::new(CreateColorantRequest {
        org_id: org_id.to_string(),
        name: name.to_string(),
    });
    client.create_colorant(request).await?;
    Ok(())
}

pub async fn describe(
    config: conf::cli::Config,
    org_id: &str,
    id: &str,
) -> Result<(), Box<dyn Error>> {
    let channel = tonic::transport::Channel::from_shared(config.server.to_string())?;
    let conn = channel.connect().await?;

    let mut client = BasecoatClient::new(conn);
    let request = tonic::Request::new(DescribeColorantRequest {
        org_id: org_id.to_string(),
        id: id.to_string(),
    });
    let response = client.describe_colorant(request).await?;
    println!("{:?}", response.into_inner().colorant);
    Ok(())
}

pub async fn delete(
    config: conf::cli::Config,
    org_id: &str,
    id: &str,
) -> Result<(), Box<dyn Error>> {
    let channel = tonic::transport::Channel::from_shared(config.server.to_string())?;
    let conn = channel.connect().await?;

    let mut client = BasecoatClient::new(conn);
    let request = tonic::Request::new(DeleteColorantRequest {
        org_id: org_id.to_string(),
        id: id.to_string(),
    });
    client.delete_colorant(request).await?;
    Ok(())
}
