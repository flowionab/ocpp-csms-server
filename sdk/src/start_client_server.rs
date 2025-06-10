use crate::ocpp_csms_server_client::csms_server_client_server::{
    CsmsServerClient, CsmsServerClientServer,
};
use std::net::SocketAddr;
use tonic::transport::Server;

pub async fn start_client_server<T: CsmsServerClient>(
    addr: SocketAddr,
    service: T,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    Server::builder()
        .add_service(CsmsServerClientServer::new(service))
        .serve_with_shutdown(addr, async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to install Ctrl+C handler");
        })
        .await?;

    Ok(())
}
