use anyhow::{Context, Result};
use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub auth_secret: String,
    pub port: u16,
    pub dev_mode: bool,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let dev_mode = env::var("DEV_MODE")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        Ok(Self {
            database_url: env::var("DATABASE_URL").context("DATABASE_URL must be set")?,
            auth_secret: env::var("AUTH_SECRET").context("AUTH_SECRET must be set")?,
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .context("PORT must be a valid number")?,
            dev_mode,
        })
    }
}
