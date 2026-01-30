use crate::network_interface::OcppProtocol;
use crate::network_interface::json::ocpp_network_interface_handle::OcppNetworkInterfaceHandle;
use serde_json::Value;
use std::sync::Arc;
use tracing::info;

pub async fn handle_ocpp_2_0_1_call_result(
    handle: &Arc<OcppNetworkInterfaceHandle>,
    raw_payload: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let (_, message_id, payload): (i64, String, Value) = serde_json::from_str(raw_payload)?;

    info!(
        protocol = OcppProtocol::Ocpp2_0_1.to_string(),
        message_id = &message_id.to_string(),
        raw_payload = &raw_payload,
        "Received call result <--"
    );

    handle.response_ocpp_2_0_1(&message_id, Ok(payload)).await;

    Ok(())
}
