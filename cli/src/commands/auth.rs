use anyhow::{Context, Result};
use clap::Subcommand;
use std::fs;
use std::path::PathBuf;

use crate::api::Config;

#[derive(Subcommand)]
pub enum AuthCommands {
    Login {
        #[arg(long)]
        token: Option<String>,
    },
    Status,
    Logout,
}

pub async fn handle(command: AuthCommands) -> Result<()> {
    match command {
        AuthCommands::Login { token } => login(token).await,
        AuthCommands::Status => status().await,
        AuthCommands::Logout => logout().await,
    }
}

async fn login(token: Option<String>) -> Result<()> {
    let token = match token {
        Some(t) => t,
        None => {
            println!("To get an API token:");
            println!("1. Go to your RabbitBench dashboard settings");
            println!("2. Create a new API token");
            println!("3. Run: rabbitbench auth login --token <your-token>");
            println!();
            println!("Or set the RABBITBENCH_TOKEN environment variable.");
            return Ok(());
        }
    };

    let config = Config { token };
    let config_path = get_config_path()?;

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let config_str = toml::to_string_pretty(&config)?;
    fs::write(&config_path, config_str)?;

    println!("Token saved to {:?}", config_path);
    println!("You are now authenticated.");

    Ok(())
}

async fn status() -> Result<()> {
    match Config::load() {
        Ok(_) => {
            println!("Authenticated (token found)");
        }
        Err(_) => {
            println!("Not authenticated");
            println!("Run 'rabbitbench auth login --token <token>' to authenticate");
        }
    }
    Ok(())
}

async fn logout() -> Result<()> {
    let config_path = get_config_path()?;

    if config_path.exists() {
        fs::remove_file(&config_path)?;
        println!("Logged out successfully");
    } else {
        println!("Not logged in");
    }

    Ok(())
}

fn get_config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Could not determine config directory")?
        .join("rabbitbench");
    Ok(config_dir.join("config.toml"))
}
