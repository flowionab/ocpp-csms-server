mod util;
mod ocpp;
mod charger;

use std::convert::Infallible;
use tracing::info;
use crate::ocpp::start_ocpp_server;
use crate::util::configure_tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    configure_tracing()?;
    info!("starting up csms server");
    start_ocpp_server().await?;

    Ok(())
}