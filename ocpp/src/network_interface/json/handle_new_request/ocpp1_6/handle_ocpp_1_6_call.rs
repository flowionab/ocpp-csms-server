use crate::network_interface::json::handle_new_request::ocpp1_6::perform_ocpp_1_6_call::perform_ocpp_1_6_call;
use crate::network_interface::json::handle_new_request::{
    OCPP_CALLS, OCPP_CALL_RESULT, OCPP_ERROR,
};
use crate::network_interface::json::ocpp_network_interface_handle::OcppNetworkInterfaceHandle;
use crate::network_interface::ocpp1_6_request_receiver::Ocpp16RequestReceiver;
use crate::network_interface::OcppProtocol;
use ocpp_client::ocpp_1_6::OCPP1_6Error;
use serde_json::Value;
use shared::Config;
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};

pub async fn handle_ocpp_1_6_call<T: Ocpp16RequestReceiver + Send + Sync + 'static>(
    config: Arc<Config>,
    charger_id: &str,
    charger: Arc<Mutex<T>>,
    handle: &Arc<OcppNetworkInterfaceHandle>,
    raw_payload: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let (_, message_id, action, payload): (i64, String, String, Value) =
        serde_json::from_str(raw_payload)?;

    let timer = OCPP_CALLS
        .with_label_values(&[OcppProtocol::Ocpp1_6.to_string(), action.clone()])
        .start_timer();

    let duration = core::time::Duration::from_secs(
        config
            .ocpp
            .clone()
            .unwrap_or_default()
            .message_timeout_secs
            .unwrap_or(30),
    );

    let response: Result<Value, OCPP1_6Error> = {
        let mut lock = charger.lock().await;

        info!(
            charger_id = charger_id,
            protocol = OcppProtocol::Ocpp1_6.to_string(),
            message_id = &message_id,
            action = &action,
            payload = payload.to_string(),
            "Incoming call <--"
        );

        perform_ocpp_1_6_call(duration, lock.deref_mut(), &action, payload).await?
    };

    match response {
        Ok(payload) => {
            info!(
                charger_id = charger_id,
                protocol = OcppProtocol::Ocpp1_6.to_string(),
                message_id = &message_id,
                action = &action,
                payload = &payload.to_string(),
                "Responding to call -->"
            );
            {
                handle
                    .send_raw(serde_json::to_string(&(
                        OCPP_CALL_RESULT,
                        message_id.to_string(),
                        payload,
                    ))?)
                    .await?;
            }
            let charger = charger.clone();
            let charger_id = charger_id.to_string();
            tokio::spawn(async move {
                let mut lock = charger.lock().await;
                if let Err(err) = lock.post_request(&action).await {
                    warn!(
                        charger_id = charger_id,
                        protocol = OcppProtocol::Ocpp1_6.to_string(),
                        message_id = &message_id,
                        action = &action,
                        error_message = err.to_string(),
                        "Failed to handle post hook"
                    );
                }
            });
        }
        Err(error) => {
            warn!(
                charger_id = charger_id,
                protocol = OcppProtocol::Ocpp1_6.to_string(),
                message_id = &message_id,
                action = &action,
                error_code = error.code(),
                error_description = error.description(),
                "Responding to call with error -->"
            );
            handle
                .send_raw(serde_json::to_string(&(
                    OCPP_ERROR,
                    message_id,
                    error.code(),
                    error.description(),
                    error.details(),
                ))?)
                .await?;
        }
    }
    timer.observe_duration();

    Ok(())
}
