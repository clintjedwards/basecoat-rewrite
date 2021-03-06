use crate::conf;
use crate::proto::basecoat_client::BasecoatClient;
use crate::proto::{
    CreateOrganizationRequest, DescribeOrganizationRequest, ListOrganizationsRequest,
};
use clap::{Args, Subcommand};
use std::error::Error;

#[derive(Debug, Args)]
pub struct OrganizationSubcommands {
    #[clap(subcommand)]
    pub command: OrganizationCommands,
}

#[derive(Debug, Subcommand)]
pub enum OrganizationCommands {
    /// Create a new organization.
    Create { name: String },

    /// List all organizations.
    List,

    /// Get details on a specific organization.
    Describe { id: String },
}

pub async fn create(config: conf::cli::Config, name: &str) -> Result<(), Box<dyn Error>> {
    let channel = tonic::transport::Channel::from_shared(config.server.to_string())?;
    let conn = channel.connect().await?;

    let mut client = BasecoatClient::new(conn);
    let request = tonic::Request::new(CreateOrganizationRequest {
        name: name.to_string(),
    });
    client.create_organization(request).await?;
    Ok(())
}

pub async fn list(config: conf::cli::Config) -> Result<(), Box<dyn Error>> {
    let channel = tonic::transport::Channel::from_shared(config.server.to_string())?;
    let conn = channel.connect().await?;

    let mut client = BasecoatClient::new(conn);
    let request = tonic::Request::new(ListOrganizationsRequest {});
    let response = client.list_organizations(request).await?;
    println!("{:?}", response.into_inner().organizations);
    Ok(())
}

pub async fn describe(config: conf::cli::Config, id: &str) -> Result<(), Box<dyn Error>> {
    let channel = tonic::transport::Channel::from_shared(config.server.to_string())?;
    let conn = channel.connect().await?;

    let mut client = BasecoatClient::new(conn);
    let request = tonic::Request::new(DescribeOrganizationRequest { id: id.to_string() });
    let response = client.describe_organization(request).await?;
    println!("{:?}", response.into_inner().organization);
    Ok(())
}
