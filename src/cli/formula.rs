use crate::conf;
use crate::proto::basecoat_client::BasecoatClient;
use crate::proto::{
    Base, Colorant, CreateFormulaRequest, DeleteFormulaRequest, DescribeFormulaRequest,
    ListFormulasRequest,
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
        /// Optional associated number for formula.
        number: Option<String>,
        /// Small notes about the formula.
        notes: Option<String>,
        /// List of base ids you want to attach to the formula.
        bases: Vec<String>,
        /// List of colorants ids you want to attach to the formula.
        colorants: Vec<String>,
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
    number: Option<String>,
    notes: Option<String>,
    bases: Vec<String>,
    colorants: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let channel = tonic::transport::Channel::from_shared(config.server.to_string())?;
    let conn = channel.connect().await?;

    let parsed_bases = bases
        .into_iter()
        .map(|base| {
            let base_info: Vec<&str> = base.split(':').collect();
            Base {
                id: base_info[0].to_string(),
                name: "".to_string(),
                amount: base_info[1].to_string(),
                manufacturer: "".to_string(),
                created: 0,
                modified: 0,
                org_id: "".to_string(),
            }
        })
        .collect();

    let parsed_colorants = colorants
        .into_iter()
        .map(|colorant| {
            let colorant_info: Vec<&str> = colorant.split(':').collect();
            Colorant {
                id: colorant_info[0].to_string(),
                name: "".to_string(),
                amount: colorant_info[1].to_string(),
                manufacturer: "".to_string(),
                created: 0,
                modified: 0,
                org_id: "".to_string(),
            }
        })
        .collect();

    let mut client = BasecoatClient::new(conn);
    let request = tonic::Request::new(CreateFormulaRequest {
        org_id: org_id.to_string(),
        name: name.to_string(),
        number: number.unwrap_or_default(),
        notes: notes.unwrap_or_default(),
        bases: parsed_bases,
        colorants: parsed_colorants,
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
