use crate::conf;
use crate::proto::basecoat_client::BasecoatClient;
use crate::proto::{
    CreateFormulaRequest, DeleteFormulaRequest, DescribeFormulaRequest, ListFormulasRequest,
};
use clap::{Args, Subcommand};
use std::error::Error;

#[derive(Debug, Args)]
pub struct FormulaSubcommands {
    #[clap(subcommand)]
    pub command: FormulaCommands,
}

#[derive(Debug, Subcommand)]
pub enum FormulaCommands {
    /// List all formulas.
    List {
        /// Organization ID of target organization.
        org_id: String,
    },

    /// Create a new formula.
    Create {
        /// Organization ID of target organization.
        org_id: String,
        /// Full name of formula.
        name: String,
    },

    /// Get details about a specific formula.
    Describe {
        /// Organization ID of target organization.
        org_id: String,
        /// ID of formula.
        id: String,
    },

    /// Delete a specific formula.
    Delete { org_id: String, id: String },
}

pub async fn list(config: conf::cli::Config, org_id: &str) -> Result<(), Box<dyn Error>> {
    let channel = tonic::transport::Channel::from_shared(config.server.to_string())?;
    let conn = channel.connect().await?;

    let mut client = BasecoatClient::new(conn);
    let request = tonic::Request::new(ListFormulasRequest {
        org_id: org_id.to_string(),
    });
    let response = client.list_formulas(request).await?;
    println!("{:?}", response.into_inner().formulas);
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
    let request = tonic::Request::new(CreateFormulaRequest {
        org_id: org_id.to_string(),
        name: name.to_string(),
    });
    client.create_formula(request).await?;
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
    let request = tonic::Request::new(DescribeFormulaRequest {
        org_id: org_id.to_string(),
        id: id.to_string(),
    });
    let response = client.describe_formula(request).await?;
    println!("{:?}", response.into_inner().formula);
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
    let request = tonic::Request::new(DeleteFormulaRequest {
        org_id: org_id.to_string(),
        id: id.to_string(),
    });
    client.delete_formula(request).await?;
    Ok(())
}
