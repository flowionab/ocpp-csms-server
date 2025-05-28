use crate::network_interface::charger_factory::ChargerFactory;
use crate::network_interface::json::authentication_handler::AuthenticationHandler;
use crate::network_interface::json::metrics_handler::metrics_handler;
use crate::network_interface::json::ocpp_handler::ocpp_handler;
use crate::network_interface::network_interface::NetworkInterface;
use crate::network_interface::ocpp1_6_request_receiver::Ocpp16RequestReceiver;
use crate::network_interface::ocpp2_0_1_request_receiver::Ocpp2_0_1RequestReceiver;
use poem::listener::TcpListener;
use poem::{get, EndpointExt, Route, Server};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal::unix::{signal, SignalKind};
use tracing::info;

pub struct OcppJsonNetworkInterface<T> {
    charger_factory: Arc<dyn ChargerFactory<T> + Send + Sync>,
    host: String,
    port: String,
}

impl<
        T: Send
            + Sync
            + AuthenticationHandler
            + Ocpp16RequestReceiver
            + Ocpp2_0_1RequestReceiver
            + 'static,
    > OcppJsonNetworkInterface<T>
{
    pub fn new<F: ChargerFactory<T> + Send + Sync + 'static>(
        charger_factory: F,
        host: &str,
        port: &str,
    ) -> Self {
        Self {
            charger_factory: Arc::new(charger_factory),
            host: host.to_string(),
            port: port.to_string(),
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let app = Route::new().at("/metrics", get(metrics_handler)).at(
            "/:id",
            get::<ocpp_handler<T>>(ocpp_handler::default()).data(Arc::clone(&self.charger_factory)),
        );

        info!(
            "starting 'OCPP Json Interface' at http://{}:{}",
            self.host, self.port
        );
        let addr: SocketAddr = format!("{}:{}", self.host, self.port).parse().unwrap();

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
}

impl<T> NetworkInterface for OcppJsonNetworkInterface<T> {}
