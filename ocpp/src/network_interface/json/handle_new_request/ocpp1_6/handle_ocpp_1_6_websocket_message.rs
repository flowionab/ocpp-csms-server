use crate::network_interface::json::handle_new_request::ocpp1_6::handle_ocpp_1_6_call::handle_ocpp_1_6_call;
use crate::network_interface::json::handle_new_request::ocpp1_6::handle_ocpp_1_6_call_result::handle_ocpp_1_6_call_result;
use crate::network_interface::json::handle_new_request::ocpp1_6::handle_ocpp_1_6_error::handle_ocpp_1_6_error;
use crate::network_interface::json::handle_new_request::{OCPP_CALL, OCPP_CALL_RESULT, OCPP_ERROR};
use crate::network_interface::json::ocpp_network_interface_handle::OcppNetworkInterfaceHandle;
use crate::network_interface::ocpp1_6_request_receiver::Ocpp16RequestReceiver;
use serde_json::Value;
use shared::Config;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_ocpp_1_6_websocket_message<T: Ocpp16RequestReceiver + Send + Sync + 'static>(
    config: Arc<Config>,
    charger_id: &str,
    charger: Arc<Mutex<T>>,
    raw_payload: &str,
    handle: &Arc<OcppNetworkInterfaceHandle>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let parts: Vec<Value> = serde_json::from_str(raw_payload)?;

    let raw_message_type = parts
        .first()
        .ok_or("The message kind part of the payload was missing")?;
    let message_type = raw_message_type
        .as_i64()
        .ok_or("The message kind part of the payload was not a number")?;

    match message_type {
        OCPP_CALL => {
            handle_ocpp_1_6_call(config, charger_id, charger, handle, raw_payload).await?;
        }
        OCPP_CALL_RESULT => {
            handle_ocpp_1_6_call_result(handle, raw_payload).await?;
        }
        OCPP_ERROR => {
            handle_ocpp_1_6_error(handle, raw_payload).await?;
        }
        _ => {
            // The ocpp spec says we should ignore unknown message types
        }
    }
    Ok(())
}
