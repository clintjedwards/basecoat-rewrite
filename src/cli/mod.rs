mod base;
mod colorant;
mod formula;
mod organization;
mod service;
mod user;
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

    /// Manages user related commands. Most requests are admin only.
    User(user::UserSubcommands),

    /// Manages formula related commands.
    Formula(formula::FormulaSubcommands),

    /// Manages colorant related commands.
    Colorant(colorant::ColorantSubcommands),

    /// Manages base related commands.
    Base(base::BaseSubcommands),
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
                organization::OrganizationCommands::List => {
                    organization::list(config).await.unwrap();
                }
                organization::OrganizationCommands::Describe { id } => {
                    organization::describe(config, &id).await.unwrap();
                }
            }
        }

        Commands::User(user) => {
            let user_cmds = user.command;
            match user_cmds {
                user::UserCommands::Create { org_id, name, pass } => {
                    user::create(config, &org_id, &name, &pass).await.unwrap();
                }
                user::UserCommands::Describe { org_id, id } => {
                    user::describe(config, &org_id, &id).await.unwrap();
                }
                user::UserCommands::List { org_id } => {
                    user::list(config, &org_id).await.unwrap();
                }
                user::UserCommands::ResetPassword { org_id, id, pass } => {
                    user::reset_password(config, &org_id, &id, &pass)
                        .await
                        .unwrap();
                }
                user::UserCommands::ToggleState { org_id, id } => {
                    user::toggle_user_state(config, &org_id, &id).await.unwrap();
                }
            }
        }

        Commands::Formula(formula) => {
            let formula_cmds = formula.command;
            match formula_cmds {
                formula::FormulaCommands::Create {
                    org_id,
                    name,
                    number,
                    notes,
                    bases,
                    colorants,
                } => {
                    formula::create(config, &org_id, &name, number, notes, bases, colorants)
                        .await
                        .unwrap();
                }
                formula::FormulaCommands::List { org_id } => {
                    formula::list(config, &org_id).await.unwrap();
                }
                formula::FormulaCommands::Describe { org_id, id } => {
                    formula::describe(config, &org_id, &id).await.unwrap();
                }
                formula::FormulaCommands::Delete { org_id, id } => {
                    formula::delete(config, &org_id, &id).await.unwrap();
                }
            }
        }

        Commands::Colorant(colorant) => {
            let colorant_cmds = colorant.command;
            match colorant_cmds {
                colorant::ColorantCommands::Create { org_id, name } => {
                    colorant::create(config, &org_id, &name).await.unwrap();
                }
                colorant::ColorantCommands::List { org_id } => {
                    colorant::list(config, &org_id).await.unwrap();
                }
                colorant::ColorantCommands::Describe { org_id, id } => {
                    colorant::describe(config, &org_id, &id).await.unwrap();
                }
                colorant::ColorantCommands::Delete { org_id, id } => {
                    colorant::delete(config, &org_id, &id).await.unwrap();
                }
            }
        }

        Commands::Base(base) => {
            let base_cmds = base.command;
            match base_cmds {
                base::BaseCommands::Create { org_id, name } => {
                    base::create(config, &org_id, &name).await.unwrap();
                }
                base::BaseCommands::List { org_id } => {
                    base::list(config, &org_id).await.unwrap();
                }
                base::BaseCommands::Describe { org_id, id } => {
                    base::describe(config, &org_id, &id).await.unwrap();
                }
                base::BaseCommands::Delete { org_id, id } => {
                    base::delete(config, &org_id, &id).await.unwrap();
                }
            }
        }
    }
}
