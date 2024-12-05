mod charger;
mod ocpp;
mod server;

use crate::charger::ChargerPool;
use crate::ocpp::start_ocpp_server;
use crate::server::start_server;
use shared::{configure_tracing, SqlxDataStore};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::try_join;
use tracing::info;

pub mod ocpp_csms_server {
    tonic::include_proto!("ocpp_csms_server");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    configure_tracing()?;

    info!("starting up csms server");

    info!("connecting to database");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/postgres")
        .await?;

    let data_store = Arc::new(SqlxDataStore::setup(pool).await?);

    let charger_pool = ChargerPool::new();

    try_join!(
        start_ocpp_server(data_store, &charger_pool),
        start_server(&charger_pool)
    )?;

    Ok(())
}
