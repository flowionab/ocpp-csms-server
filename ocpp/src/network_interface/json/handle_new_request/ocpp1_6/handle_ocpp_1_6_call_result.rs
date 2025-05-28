use crate::network_interface::json::ocpp_network_interface_handle::OcppNetworkInterfaceHandle;
use crate::network_interface::OcppProtocol;
use serde_json::Value;
use std::sync::Arc;
use tracing::info;

pub async fn handle_ocpp_1_6_call_result(
    handle: &Arc<OcppNetworkInterfaceHandle>,
    raw_payload: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let (_, message_id, payload): (i64, String, Value) = serde_json::from_str(raw_payload)?;

    info!(
        protocol = OcppProtocol::Ocpp1_6.to_string(),
        message_id = &message_id.to_string(),
        raw_payload = &raw_payload,
        "Received call result <--"
    );

    handle.response_ocpp_1_6(&message_id, Ok(payload)).await;

    Ok(())
}
