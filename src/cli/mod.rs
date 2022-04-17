mod organization;
mod service;
use std::str::FromStr;

use crate::conf;
use clap::{Parser, Subcommand};
use slog::o;
use sloggers::terminal::{Destination, TerminalLoggerBuilder};
use sloggers::types::Severity;
use sloggers::Build;

#[derive(Debug, Parser)]
#[clap(name = "basecoat")]
#[clap(about = "Basecoat is a formula tracking and search tool")]
#[clap(version)]
struct Cli {
    /// Set configuration path; if empty default paths are used
    #[clap(long, value_name = "PATH")]
    config: Option<String>,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Manages service related commands pertaining to administration.
    Service(service::ServiceSubcommands),

    /// Manages organization related commands. Most requests are admin only.
    Organization(organization::OrganizationSubcommands),
}

fn init_logging(severity: Severity) -> slog_scope::GlobalLoggerGuard {
    let mut builder = TerminalLoggerBuilder::new();
    builder.level(severity);
    builder.destination(Destination::Stderr);

    let root_logger = builder.build().unwrap();
    let log = slog::Logger::root(root_logger, o!());

    slog_scope::set_global_logger(log)
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
                        let severity =
                            sloggers::types::Severity::from_str(&parsed_config.general.log_level)
                                .expect(
                                    "could not parse log_level; must be one of
                                ['trace', 'debug', 'info', 'warning', 'error', 'critical']",
                                );
                        let _guard = init_logging(severity);
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
        Commands::Organization(org) => {
            let org_cmds = org.command;
            match org_cmds {
                organization::OrganizationCommands::Create { name } => {
                    organization::create(config, &name).await.unwrap();
                }
            }
        }
    }
}
