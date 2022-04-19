use crate::conf;
use crate::proto::basecoat_client::BasecoatClient;
use crate::proto::{
    CreateUserRequest, DescribeUserRequest, ListUsersRequest, ResetUserPasswordRequest,
    ToggleUserStateRequest,
};
use clap::{Args, Subcommand};
use std::error::Error;

#[derive(Debug, Args)]
pub struct UserSubcommands {
    #[clap(subcommand)]
    pub command: UserCommands,
}

#[derive(Debug, Subcommand)]
pub enum UserCommands {
    /// List all users.
    List {
        /// Organization ID of target organization.
        org_id: String,
    },

    /// Create a new user.
    Create {
        /// Organization ID of target organization.
        org_id: String,
        /// Full name of user.
        name: String,
        /// Password of user.
        pass: String,
    },

    /// Get details about a specific user.
    Describe {
        /// Organization ID of target organization.
        org_id: String,
        /// ID of user.
        id: String,
    },

    /// Reset a user's password.
    ResetPassword {
        /// Organization ID of target organization.
        org_id: String,
        /// ID of user.
        id: String,
        /// New password.
        pass: String,
    },

    /// Toggle the state of a user.
    ToggleState {
        /// Organization ID of target organization.
        org_id: String,
        /// ID of user.
        id: String,
    },
}

pub async fn list(config: conf::cli::Config, org_id: &str) -> Result<(), Box<dyn Error>> {
    let channel = tonic::transport::Channel::from_shared(config.server.to_string())?;
    let conn = channel.connect().await?;

    let mut client = BasecoatClient::new(conn);
    let request = tonic::Request::new(ListUsersRequest {
        org_id: org_id.to_string(),
    });
    let response = client.list_users(request).await?;
    println!("{:?}", response.into_inner().users);
    Ok(())
}

pub async fn create(
    config: conf::cli::Config,
    org_id: &str,
    name: &str,
    pass: &str,
) -> Result<(), Box<dyn Error>> {
    let channel = tonic::transport::Channel::from_shared(config.server.to_string())?;
    let conn = channel.connect().await?;

    let mut client = BasecoatClient::new(conn);
    let request = tonic::Request::new(CreateUserRequest {
        org_id: org_id.to_string(),
        name: name.to_string(),
        password: pass.to_string(),
    });
    client.create_user(request).await?;
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
    let request = tonic::Request::new(DescribeUserRequest {
        org_id: org_id.to_string(),
        id: id.to_string(),
    });
    let response = client.describe_user(request).await?;
    println!("{:?}", response.into_inner().user);
    Ok(())
}

pub async fn reset_password(
    config: conf::cli::Config,
    org_id: &str,
    id: &str,
    pass: &str,
) -> Result<(), Box<dyn Error>> {
    let channel = tonic::transport::Channel::from_shared(config.server.to_string())?;
    let conn = channel.connect().await?;

    let mut client = BasecoatClient::new(conn);
    let request = tonic::Request::new(ResetUserPasswordRequest {
        org_id: org_id.to_string(),
        id: id.to_string(),
        password: pass.to_string(),
    });
    client.reset_user_password(request).await?;
    Ok(())
}

pub async fn toggle_user_state(
    config: conf::cli::Config,
    org_id: &str,
    id: &str,
) -> Result<(), Box<dyn Error>> {
    let channel = tonic::transport::Channel::from_shared(config.server.to_string())?;
    let conn = channel.connect().await?;

    let mut client = BasecoatClient::new(conn);
    let request = tonic::Request::new(ToggleUserStateRequest {
        org_id: org_id.to_string(),
        id: id.to_string(),
    });
    client.toggle_user_state(request).await?;
    Ok(())
}
