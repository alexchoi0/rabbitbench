use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod adapters;
mod api;
mod commands;

use commands::{auth, project, run};

#[derive(Parser)]
#[command(name = "rabbitbench")]
#[command(about = "CLI tool for benchmark submission and management")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(
        long,
        env = "RABBITBENCH_API_URL",
        default_value = "http://localhost:8080"
    )]
    api_url: String,
}

#[derive(Subcommand)]
enum Commands {
    Auth {
        #[command(subcommand)]
        command: auth::AuthCommands,
    },
    Project {
        #[command(subcommand)]
        command: project::ProjectCommands,
    },
    Run(run::RunArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rabbitbench=info".into()),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Auth { command } => auth::handle(command).await,
        Commands::Project { command } => project::handle(command, &cli.api_url).await,
        Commands::Run(args) => run::handle(args, &cli.api_url).await,
    }
}
