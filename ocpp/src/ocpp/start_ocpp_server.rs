use crate::charger::ChargerPool;
use crate::ocpp::ocpp_handler::ocpp_handler;
use poem::listener::TcpListener;
use poem::{get, EndpointExt, Route, Server};
use shared::{Config, DataStore};
use std::sync::Arc;
use tokio::signal::unix::{signal, SignalKind};
use tracing::{info, instrument};

#[instrument(skip_all)]
pub async fn start_ocpp_server(
    config: &Config,
    data_store: Arc<dyn DataStore>,
    charger_pool: &ChargerPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let app = Route::new().at(
        "/:id",
        get(ocpp_handler).data((config.clone(), data_store, charger_pool.clone())),
    );
    Server::new(TcpListener::bind("192.168.1.83:50051"))
        .run_with_graceful_shutdown(
            app,
            async {
                signal(SignalKind::terminate()).unwrap().recv().await;
                info!("Shutting down server")
            },
            None,
        )
        .await?;
    Ok(())
}
