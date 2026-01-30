use crate::network_interface::OcppProtocol;
use crate::network_interface::json::handle_new_request::ocpp1_6::parse_ocpp_1_6_error_payload::parse_ocpp_1_6_error_payload;
use crate::network_interface::json::ocpp_network_interface_handle::OcppNetworkInterfaceHandle;
use std::sync::Arc;
use tracing::warn;

pub async fn handle_ocpp_1_6_error(
    handle: &Arc<OcppNetworkInterfaceHandle>,
    raw_payload: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let (message_id, error) = parse_ocpp_1_6_error_payload(raw_payload)?;

    warn!(
        protocol = OcppProtocol::Ocpp1_6.to_string(),
        message_id = &message_id.to_string(),
        raw_payload = &raw_payload,
        "Received error <--"
    );

    handle.response_ocpp_1_6(&message_id, Err(error)).await;

    Ok(())
}
