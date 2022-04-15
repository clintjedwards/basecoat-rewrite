mod service;
use crate::conf;
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
    let config;
    match conf::Kind::new_cli_config().parse(&args.config).unwrap() {
        conf::Kind::Cli(parsed_config) => config = parsed_config,
        _ => {
            panic!("Incorrect configuration file received")
        }
    }

    match args.command {
        Commands::Service(service) => {
            let service_cmds = service.command;
            match service_cmds {
                service::ServiceCommands::Start => {
                    if let conf::Kind::Api(parsed_config) =
                        conf::Kind::new_api_config().parse(&args.config).unwrap()
                    {
                        service::start(parsed_config).await;
                    } else {
                        panic!("Incorrect configuration file received trying to start api")
                    }
                }
                service::ServiceCommands::Info => {
                    service::info(config).await.expect("could not get info");
                }
            }
        }
    }
}
