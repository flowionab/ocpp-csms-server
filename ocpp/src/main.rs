#![allow(clippy::module_inception)]

mod charger;
mod event;
mod ocpp;
mod server;

use crate::charger::ChargerPool;
use crate::event::EventManager;
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
    let _ = dotenvy::dotenv();
    configure_tracing()?;
    info!("starting up csms server");
    let config = read_config().await?;
    config.print_config_warnings();

    let data_store = {
        info!("connecting to database");

        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            warn!(
                "No DATABASE_URL env var provided, attempting to connect to database at localhost"
            );
            "postgres://postgres:password@localhost/ocpp_csms_server".to_string()
        });
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(std::time::Duration::from_secs(5))
            .connect(&database_url)
            .await
            .inspect_err(|e| {
                error!(
                    error_message = e.to_string(),
                    "Could not connect to database"
                );
            })?;

        Arc::new(SqlxDataStore::setup(pool).await?)
    };

    let event_manager = EventManager::from_config(&config).await?;

    let charger_pool = ChargerPool::new();

    try_join!(
        start_ocpp_server(&config, data_store, &charger_pool, event_manager),
        start_server(&charger_pool)
    )?;

    Ok(())
}
