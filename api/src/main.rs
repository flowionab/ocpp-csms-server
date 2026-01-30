use shared::configure_tracing;
use shared::data_store::MongoDataStore;
use std::env;
use tracing::{info, warn};

pub mod ocpp_csms_server {
    tonic::include_proto!("ocpp_csms_server");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    configure_tracing()?;

    info!("starting up api server");

    info!("connecting to database");
    let mongo_connection_uri = env::var("MONGO_CONNECTION_URI").unwrap_or_else(|_| {
        warn!("No MONGO_CONNECTION_URI env var provided, attempting to connect to database at localhost");
        "mongodb://localhost:27017".to_string()
    });

    let _data_store = Box::new(MongoDataStore::setup(mongo_connection_uri).await?);

    Ok(())
}
