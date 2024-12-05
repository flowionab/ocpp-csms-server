use crate::start_server::start_server;
use shared::{configure_tracing, SqlxDataStore};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing::info;

mod api_service;
mod start_server;

pub mod ocpp_csms_server {
    tonic::include_proto!("ocpp_csms_server");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    configure_tracing()?;

    info!("starting up api server");

    info!("connecting to database");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/postgres")
        .await?;

    let data_store = Box::new(SqlxDataStore::setup(pool).await?);

    start_server(data_store).await?;

    Ok(())
}
