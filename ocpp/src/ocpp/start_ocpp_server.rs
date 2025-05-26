use crate::charger::ChargerPool;
use crate::event::EventManager;
use crate::ocpp::ocpp_handler::ocpp_handler;
use poem::listener::TcpListener;
use poem::{get, EndpointExt, Route, Server};
use shared::{Config, DataStore};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal::unix::{signal, SignalKind};
use tracing::info;

pub async fn start_ocpp_server(
    config: &Config,
    data_store: Arc<dyn DataStore>,
    charger_pool: &ChargerPool,
    event_manager: EventManager,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let node_address =
        env::var("NODE_ADDRESS").unwrap_or_else(|_| "http://localhost:50052".to_string());

    let easee_master_password: Option<String> =
        env::var("EASEE_MASTER_PASSWORD").ok().map(|password| {
            hex::decode(password)
                .map(|bytes| String::from_utf8(bytes).unwrap())
                .expect("Could not decode EASEE_MASTER_PASSWORD")
        });

    let app = Route::new().at(
        "/:id",
        get(ocpp_handler).data((
            config.clone(),
            data_store,
            charger_pool.clone(),
            node_address.clone(),
            easee_master_password.clone(),
            event_manager.clone(),
        )),
    );

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("OCPP_PORT").unwrap_or_else(|_| "50051".to_string());
    let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();

    info!("starting OCPP server at http://{}:{}", host, port);
    info!("node address used is {}", node_address);

    Server::new(TcpListener::bind(addr))
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
