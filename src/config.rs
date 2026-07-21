use std::env;

use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub listen_addr: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Self {
            database_url: env::var("DATABASE_URL").context("DATABASE_URL must be set")?,
            jwt_secret: env::var("JWT_SECRET").context("JWT_SECRET must be set")?,
            listen_addr: env::var("LISTEN_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into()),
        })
    }

    pub fn for_test() -> Self {
        Self {
            database_url: ":memory:".into(),
            jwt_secret: "test-secret-do-not-use-in-production".into(),
            listen_addr: "127.0.0.1:0".into(),
        }
    }
}
