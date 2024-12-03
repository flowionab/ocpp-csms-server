mod util;
mod ocpp;
mod charger;
mod data;

use std::sync::Arc;
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use crate::data::SqlxDataStore;
use crate::ocpp::start_ocpp_server;
use crate::util::configure_tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    configure_tracing()?;

    info!("starting up csms server");

    info!("Connecting to database");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/postgres").await?;

    let data_store = Arc::new(SqlxDataStore::setup(pool).await?);
    start_ocpp_server(data_store).await?;

    Ok(())
}