use crate::charger::ChargerPool;
use crate::ocpp_csms_server::ocpp_server::OcppServer;
use crate::server::ocpp_service::OcppService;
use std::env;
use std::net::SocketAddr;
use tonic::transport::Server;
use tracing::{info, instrument};

#[instrument(skip_all)]
pub async fn start_server(
    charger_pool: &ChargerPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let (health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<OcppServer<OcppService>>()
        .await;

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("API_PORT").unwrap_or_else(|_| "50052".to_string());
    let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();

    info!(address = addr.to_string(), "starting grpc server endpoint");

    Server::builder()
        .add_service(health_service)
        .add_service(OcppServer::new(OcppService::new(charger_pool.clone())))
        .serve(addr)
        .await?;

    Ok(())
}
