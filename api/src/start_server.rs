use crate::api_service::ApiService;
use crate::ocpp_csms_server::api_server::ApiServer;
use shared::data_store::DataStore;
use tonic::transport::Server;
use tracing::{info, instrument};

#[instrument]
pub async fn start_server(
    data_store: Box<dyn DataStore>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let (health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter.set_serving::<ApiServer<ApiService>>().await;

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "50053".to_string());
    let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();

    info!(address = addr.to_string(), "starting grpc server endpoint");

    Server::builder()
        .add_service(health_service)
        .add_service(ApiServer::new(ApiService::new(data_store)))
        .serve(addr)
        .await?;

    Ok(())
}
