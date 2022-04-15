mod service;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "basecoat")]
#[clap(about = "Basecoat is a formula tracking and search tool")]
#[clap(version)]
struct Cli {
    #[clap(
        long,
        help = "Set configuration path; default to default paths if not set"
    )]
    config: Option<String>,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Manages service related commands pertaining to administration.
    Service(service::ServiceSubcommands),
}

/// init the CLI and appropriately run the correct command.
pub async fn init() {
    let args = Cli::parse();

    match args.command {
        Commands::Service(service) => {
            let service_cmds = service.command;
            match service_cmds {
                service::ServiceCommands::Start => {
                    service::start(args.config).unwrap().await;
                }
            }
        }
    }
}
