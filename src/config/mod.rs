pub mod crypto;
use actix_web::test::config;
use color_eyre::Result;
use serde::Deserialize;
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use tracing::{info,instrument};
use tracing_subscriber::EnvFilter;
use eyre::WrapErr;
use std::{time::Duration, sync::Arc};

#[derive(Debug,Deserialize)]
pub struct Config{
    pub host: String,
    pub port: i32,
    pub database_url: String,
    pub secret_key: String
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();


       info!("Loading configuration");

        let mut c = config::Config::new();

        c.merge(config::Environment::default())?;

        c.try_into()
        .context("loading configuration from environment")
    }

    pub async fn db_pool(&self) -> Result<PgPool>{
        info!("Creating database connection");
        PgPool::builder()
        .connect_timeout(Duration::from_secs(30))
        .build(&self.database_url)
        .await
        .context("Creating database connection pool")
    }

    pub fn crypto_service(&self) -> CryptoService{
        CryptoService{
            key: Arc::new(self.secret_key.clone())
        }
    }
}
