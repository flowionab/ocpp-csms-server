use crate::start_server::start_server;
use shared::{configure_tracing, SqlxDataStore};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tracing::{info, warn};

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
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        warn!("No DATABASE_URL env var provided, attempting to connect to database at localhost");
        "postgres://postgres:password@localhost/postgres".to_string()
    });
    dbg!(&database_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let data_store = Box::new(SqlxDataStore::setup(pool).await?);

    start_server(data_store).await?;

    Ok(())
}
