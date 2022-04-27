use crate::conf;
use crate::proto::basecoat_client::BasecoatClient;
use crate::proto::{CreateBaseRequest, DeleteBaseRequest, DescribeBaseRequest, ListBasesRequest};
use clap::{Args, Subcommand};
use std::error::Error;

#[derive(Debug, Args)]
pub struct BaseSubcommands {
    #[clap(subcommand)]
    pub command: BaseCommands,
}

#[derive(Debug, Subcommand)]
pub enum BaseCommands {
    /// List all bases.
    List {
        /// Organization ID of target organization.
        org_id: String,
    },

    /// Create a new base.
    Create {
        /// Organization ID of target organization.
        org_id: String,
        /// Full name of base.
        name: String,
        /// Maker of the colorant
        manufacturer: Option<String>,
    },

    /// Get details about a specific base.
    Describe {
        /// Organization ID of target organization.
        org_id: String,
        /// ID of base.
        id: String,
    },

    /// Delete a specific base.
    Delete { org_id: String, id: String },
}

pub async fn list(config: conf::cli::Config, org_id: &str) -> Result<(), Box<dyn Error>> {
    let channel = tonic::transport::Channel::from_shared(config.server.to_string())?;
    let conn = channel.connect().await?;

    let mut client = BasecoatClient::new(conn);
    let request = tonic::Request::new(ListBasesRequest {
        org_id: org_id.to_string(),
    });
    let response = client.list_bases(request).await?;
    println!("{:?}", response.into_inner().bases);
    Ok(())
}

pub async fn create(
    config: conf::cli::Config,
    org_id: &str,
    name: &str,
    manufacturer: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let channel = tonic::transport::Channel::from_shared(config.server.to_string())?;
    let conn = channel.connect().await?;

    let mut client = BasecoatClient::new(conn);
    let request = tonic::Request::new(CreateBaseRequest {
        org_id: org_id.to_string(),
        name: name.to_string(),
        manufacturer: manufacturer.unwrap_or_default(),
    });
    client.create_base(request).await?;
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
    let request = tonic::Request::new(DescribeBaseRequest {
        org_id: org_id.to_string(),
        id: id.to_string(),
    });
    let response = client.describe_base(request).await?;
    println!("{:?}", response.into_inner().base);
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
    let request = tonic::Request::new(DeleteBaseRequest {
        org_id: org_id.to_string(),
        id: id.to_string(),
    });
    client.delete_base(request).await?;
    Ok(())
}
