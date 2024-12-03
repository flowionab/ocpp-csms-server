use std::sync::Arc;
use poem::{get, EndpointExt, Route, Server};
use poem::listener::TcpListener;
use tokio::signal::unix::{signal, SignalKind};
use tracing::info;
use crate::data::DataStore;
use crate::ocpp::ocpp_handler::ocpp_handler;

pub async fn start_ocpp_server(data_store: Arc<dyn DataStore>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let app = Route::new().at("/:id", get(ocpp_handler).data(data_store));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run_with_graceful_shutdown(app, async {
            signal(SignalKind::terminate()).unwrap().recv().await;
            info!("Shutting down server")
        }, None)
        .await?;
    Ok(())
}