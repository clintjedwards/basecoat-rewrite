use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "basecoat")]
#[clap(about = "Basecoat is a formula tracking and search tool")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Manages service related commands pertaining to administration
    Service(ServiceSubcommands),
}

#[derive(Debug, Args)]
struct ServiceSubcommands {
    #[clap(subcommand)]
    command: ServiceCommands,
}

#[derive(Debug, Subcommand)]
enum ServiceCommands {
    /// Start the Basecoat GRPC/HTTP combined service
    #[clap(
        long_about = "Basecoat runs a a GRPC backend combined with GRPC-WEB/HTTP1.
    Running this command attempts to start the long running service. This command will block and only
    gracefully stop on SIGINT or SIGTERM signals."
    )]
    Start,
}

/// init the CLI and appropriately run the correct command.
pub fn init() {
    let args = Cli::parse();

    match args.command {
        Commands::Service(service) => {
            let service_cmds = service.command;
            match service_cmds {
                ServiceCommands::Start => {
                    println!("start here!")
                }
            }
        }
    }
}
