use poem::{get, Route, Server};
use poem::listener::TcpListener;
use tokio::signal::unix::{signal, SignalKind};
use tracing::info;
use crate::ocpp::ocpp_handler::ocpp_handler;

pub async fn start_ocpp_server() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let app = Route::new().at("/:id", get(ocpp_handler));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run_with_graceful_shutdown(app, async {
            signal(SignalKind::terminate()).unwrap().recv().await;
            info!("Shutting down server")
        }, None)
        .await?;
    Ok(())
}