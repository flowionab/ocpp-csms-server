#![allow(clippy::module_inception)]

mod charger;
mod ocpp;
mod server;

use crate::charger::ChargerPool;
use crate::ocpp::start_ocpp_server;
use crate::server::start_server;
use shared::{configure_tracing, read_config, SqlxDataStore};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;
use tokio::try_join;
use tracing::{error, info, warn};

pub mod ocpp_csms_server {
    tonic::include_proto!("ocpp_csms_server");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    configure_tracing()?;
    info!("starting up csms server");
    let config = read_config().await?;
    config.print_config_warnings();

    info!("connecting to database");
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        warn!("No DATABASE_URL env var provided, attempting to connect to database at localhost");
        "postgres://postgres:password@localhost/postgres".to_string()
    });
    dbg!(&database_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .inspect_err(|e| {
            error!(
                error_message = e.to_string(),
                "Could not connect to database"
            );
        })?;

    let data_store = Arc::new(SqlxDataStore::setup(pool).await?);

    let charger_pool = ChargerPool::new();

    try_join!(
        start_ocpp_server(&config, data_store, &charger_pool),
        start_server(&charger_pool)
    )?;

    Ok(())
}
