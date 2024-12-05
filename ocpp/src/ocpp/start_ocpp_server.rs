use crate::charger::ChargerPool;
use crate::ocpp::ocpp_handler::ocpp_handler;
use poem::listener::TcpListener;
use poem::{get, EndpointExt, Route, Server};
use shared::DataStore;
use std::sync::Arc;
use tokio::signal::unix::{signal, SignalKind};
use tracing::info;

pub async fn start_ocpp_server(
    data_store: Arc<dyn DataStore>,
    charger_pool: &ChargerPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let app = Route::new().at(
        "/:id",
        get(ocpp_handler).data((data_store, charger_pool.clone())),
    );
    Server::new(TcpListener::bind("0.0.0.0:3001"))
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
