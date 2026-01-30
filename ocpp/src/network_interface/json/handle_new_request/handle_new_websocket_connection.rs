use crate::network_interface::OcppProtocol;
use crate::network_interface::charger_factory::ChargerFactory;
use crate::network_interface::json::handle_new_request::handle_websocket_message::handle_websocket_message;
use crate::network_interface::json::ocpp_network_interface_handle::OcppNetworkInterfaceHandle;
use crate::network_interface::network_interface_handle::NetworkInterfaceHandle;
use crate::network_interface::ocpp1_6_request_receiver::Ocpp16RequestReceiver;
use crate::network_interface::ocpp2_0_1_request_receiver::Ocpp2_0_1RequestReceiver;
use futures::{StreamExt, TryStreamExt};
use poem::web::websocket::WebSocketStream;
use shared::Config;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use tracing::warn;

pub async fn handle_new_websocket_connection<
    T: Ocpp16RequestReceiver + Ocpp2_0_1RequestReceiver + Send + Sync + 'static,
>(
    config: Arc<Config>,
    id: &str,
    protocol: OcppProtocol,
    charger: T,
    charger_factory: &Arc<dyn ChargerFactory<T> + Send + Sync>,
    handle: &Arc<OcppNetworkInterfaceHandle>,
    socket: WebSocketStream,
) {
    info!(
        charger_id = &id,
        ocpp_protocol = protocol.to_string(),
        "websocket connection established",
    );
    let (sink, stream) = socket.split();

    handle.attach_sink(sink).await;
    let charger = Arc::new(Mutex::new(charger));

    if let Err(error) = charger_factory.on_connected(&charger).await {
        warn!(
            charger_id = &id,
            ocpp_protocol = protocol.to_string(),
            error_message = error.to_string(),
            "failed to handle charger connection"
        );
        let _ = handle.disconnect().await;
        return;
    }

    if let Err(err) = stream
        .map_err(Into::<Box<dyn std::error::Error + Send + Sync>>::into)
        .try_for_each(|message| {
            let charger = Arc::clone(&charger);
            let config = Arc::clone(&config);
            async move {
                handle_websocket_message(config, id, protocol, charger, handle, message).await?;
                Ok(())
            }
        })
        .await
    {
        info!(
            charger_id = &id,
            ocpp_protocol = protocol.to_string(),
            error_message = err.to_string(),
            "connection closed with error"
        );
        let _ = charger_factory.on_disconnected(&charger).await;
    } else {
        info!(
            charger_id = &id,
            ocpp_protocol = protocol.to_string(),
            "connection closed"
        );
        let _ = charger_factory.on_disconnected(&charger).await;
    }
}
