#![allow(clippy::module_inception)]

mod charger;
mod event;
mod server;

mod network_interface;

use crate::charger::{ChargerFactory, ChargerPool};
use crate::event::EventManager;
use crate::network_interface::json::OcppJsonNetworkInterface;
use crate::ocpp_csms_server_client::csms_server_client_client::CsmsServerClientClient;
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

pub mod ocpp_csms_server_client {
    tonic::include_proto!("ocpp_csms_server.client");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _ = dotenvy::dotenv();
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("OCPP_PORT").unwrap_or_else(|_| "50051".to_string());
    let node_address =
        env::var("NODE_ADDRESS").unwrap_or_else(|_| "http://localhost:50052".to_string());

    let easee_master_password: Option<String> =
        env::var("EASEE_MASTER_PASSWORD").ok().map(|password| {
            hex::decode(password)
                .map(|bytes| String::from_utf8(bytes).unwrap())
                .expect("Could not decode EASEE_MASTER_PASSWORD")
        });

    configure_tracing()?;
    info!("starting up csms server");
    let config = Arc::new(read_config().await?);
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

    let csms_server_client = match config.client_url() {
        None => {
            warn!("no client_url provided, not connecting to ocpp api client");
            None
        }
        Some(url) => {
            info!(url = url, "connecting to csms server client");
            let client = CsmsServerClientClient::connect(url.to_string())
                .await
                .expect("Failed to connect to OCPP API client");
            info!(url = url, "successfully connected to ocpp api client");
            Some(client)
        }
    };

    let event_manager = EventManager::from_config(&config).await?;

    let charger_pool = ChargerPool::new();

    let charger_factory = ChargerFactory::new(
        Arc::clone(&config),
        Arc::clone(&data_store) as Arc<dyn shared::DataStore + Send + Sync>,
        &node_address,
        easee_master_password,
        &event_manager,
        &charger_pool,
        &csms_server_client,
    );

    let interface =
        OcppJsonNetworkInterface::new(&config, &charger_pool, charger_factory, &host, &port);

    try_join!(interface.start(), start_server(&charger_pool))?;

    Ok(())
}
