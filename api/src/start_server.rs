use crate::api_service::ApiService;
use crate::ocpp_csms_server::api_server::ApiServer;
use shared::DataStore;
use std::net::SocketAddr;
use std::sync::Arc;
use tonic::transport::Server;
use tracing::{info, instrument};

#[instrument]
pub async fn start_server(
    data_store: Box<dyn DataStore>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter.set_serving::<ApiServer<ApiService>>().await;

    let addr: SocketAddr = "[::1]:50053".parse().unwrap();

    info!(address = addr.to_string(), "starting grpc server endpoint");

    Server::builder()
        .add_service(health_service)
        .add_service(ApiServer::new(ApiService::new(data_store)))
        .serve(addr)
        .await?;

    Ok(())
}
