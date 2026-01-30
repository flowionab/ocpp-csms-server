use crate::network_interface::OcppProtocol;
use crate::network_interface::json::handle_new_request::ocpp1_6::handle_ocpp_1_6_websocket_message;
use crate::network_interface::json::handle_new_request::ocpp2_0_1::handle_ocpp_2_0_1_websocket_message;
use crate::network_interface::json::ocpp_network_interface_handle::OcppNetworkInterfaceHandle;
use crate::network_interface::ocpp1_6_request_receiver::Ocpp16RequestReceiver;
use crate::network_interface::ocpp2_0_1_request_receiver::Ocpp2_0_1RequestReceiver;
use poem::web::websocket::Message;
use poem::web::websocket::Message::Text;
use shared::Config;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_websocket_message<
    T: Ocpp16RequestReceiver + Ocpp2_0_1RequestReceiver + Send + Sync + 'static,
>(
    config: Arc<Config>,
    charger_id: &str,
    protocol: OcppProtocol,
    charger: Arc<Mutex<T>>,
    handle: &Arc<OcppNetworkInterfaceHandle>,
    message: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Text(raw_payload) = message {
        match protocol {
            OcppProtocol::Ocpp1_6 => {
                handle_ocpp_1_6_websocket_message(config, charger_id, charger, &raw_payload, handle)
                    .await?
            }
            OcppProtocol::Ocpp2_0_1 => {
                handle_ocpp_2_0_1_websocket_message(
                    config,
                    charger_id,
                    charger,
                    &raw_payload,
                    handle,
                )
                .await?
            }
        }
    }

    Ok(())
}
