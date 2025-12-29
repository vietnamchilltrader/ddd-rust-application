use std::time::Duration;

use anyhow::Result;
use serde::Deserialize;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

#[derive(Deserialize, Clone, Debug)]
pub struct Env {
    pub database_url: String,

    #[serde(default = "default_max_connection")]
    pub max_connection: u32,

    #[serde(default = "default_min_connection")]
    pub min_connection: u32,
}

fn default_max_connection() -> u32 {
    50
}

fn default_min_connection() -> u32 {
    5
}

impl Env {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        dotenvy::dotenv().ok();
        info!("Loading configuration from environment variables");
        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        config.try_deserialize()
    }
}

pub async fn init_connection(db_url: &str, max_conn: u32, min_conn: u32) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(max_conn)
        .min_connections(min_conn)
        .acquire_timeout(Duration::from_secs(1))
        .connect(db_url)
        .await?;
    Ok(pool)
}
