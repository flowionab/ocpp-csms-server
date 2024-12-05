use std::collections::BTreeMap;
use std::sync::Arc;
use futures::SinkExt;
use futures::stream::SplitSink;
use ocpp_client::ocpp_1_6::OCPP1_6Error;
use poem::web::websocket::{Message, WebSocketStream};
use poem::web::websocket::Message::Text;
use serde_json::Value;
use tokio::sync::Mutex;
use tokio::sync::oneshot::Sender;
use tracing::{info, warn};
use crate::charger::Charger;
use crate::ocpp::ocpp_protocol::OcppProtocol;

const OCPP_CALL: i64 = 2;
const OCPP_CALL_RESULT: i64 = 3;
const OCPP_ERROR: i64 = 4;

pub async fn handle_message(charger: Arc<Mutex<Charger>>, message: Message, protocol: OcppProtocol, sink: Arc<tokio::sync::Mutex<SplitSink<WebSocketStream, Message>>>, message_queue: Arc<Mutex<BTreeMap<String, Sender<Result<Value, OCPP1_6Error>>>>>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    if let Text(raw_payload) = message {
        match protocol {
            OcppProtocol::Ocpp1_6 => {
                handle_ocpp_1_6_message(charger, &raw_payload, sink, message_queue).await?
            }
            OcppProtocol::Ocpp2_0_1 => {}
        }
    }

    Ok(())
}

async fn handle_ocpp_1_6_message(charger: Arc<Mutex<Charger>>, raw_payload: &str, sink: Arc<tokio::sync::Mutex<SplitSink<WebSocketStream, Message>>>, message_queue: Arc<Mutex<BTreeMap<String, Sender<Result<Value, OCPP1_6Error>>>>>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let parts: Vec<Value> = serde_json::from_str(&raw_payload)?;

    let raw_message_type = parts.get(0).ok_or_else(|| "The message kind part of the payload was missing")?;
    let message_type = raw_message_type.as_i64().ok_or_else(|| "The message kind part of the payload was not a number")?;
    match message_type {
        OCPP_CALL => {
            handle_ocpp_1_6_call(charger, raw_payload, sink).await?;
        }
        OCPP_CALL_RESULT => {
            handle_ocpp_1_6_call_result(message_queue, raw_payload).await?;
        }
        OCPP_ERROR => {
            handle_ocpp_1_6_error(message_queue, raw_payload).await?;
        }
        _ => {
            // The ocpp spec says we should ignore unknown message types
        }
    }
    Ok(())
}

async fn handle_ocpp_1_6_call_result(message_queue: Arc<Mutex<BTreeMap<String, Sender<Result<Value, OCPP1_6Error>>>>>, raw_payload: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let (_, message_id, payload): (i64, String, Value) = serde_json::from_str(&raw_payload)?;


    info!(
        protocol = OcppProtocol::Ocpp1_6.to_string(),
        message_id = &message_id.to_string(),
        raw_payload = &raw_payload,
        "Received call result <--"
    );

    let mut lock = message_queue.lock().await;
    if let Some(sender) = lock.remove(&message_id) {
        if let Err(_) = sender.send(Ok(payload)) {
            warn!("The message had timed out");
        }
    } else {
        warn!("We were not expecting this message, dropping it...")
    }

    Ok(())
}

async fn handle_ocpp_1_6_error(message_queue: Arc<Mutex<BTreeMap<String, Sender<Result<Value, OCPP1_6Error>>>>>, raw_payload: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let (_, message_id, code, description, details): (i64, String, String, String, Value) = serde_json::from_str(&raw_payload)?;

    warn!(
        protocol = OcppProtocol::Ocpp1_6.to_string(),
        message_id = &message_id.to_string(),
        raw_payload = &raw_payload,
        "Received error <--"
    );

    let err = match code.as_str() {
        "NotImplemented" => {
            OCPP1_6Error::NotImplemented {
                description,
                details
            }
        },
        "NotSupported" => {
            OCPP1_6Error::NotSupported {
                description,
                details
            }
        },
        "InternalError" => {
            OCPP1_6Error::InternalError {
                description,
                details
            }
        },
        "ProtocolError" => {
            OCPP1_6Error::ProtocolError {
                description,
                details
            }
        },
        "SecurityError" => {
            OCPP1_6Error::SecurityError {
                description,
                details
            }
        },
        "FormationViolation" => {
            OCPP1_6Error::FormationViolation {
                description,
                details
            }
        },
        "PropertyConstraintViolation" => {
            OCPP1_6Error::PropertyConstraintViolation {
                description,
                details
            }
        },
        "OccurenceConstraintViolation" => {
            OCPP1_6Error::OccurenceConstraintViolation {
                description,
                details
            }
        },
        "TypeConstraintViolation" => {
            OCPP1_6Error::TypeConstraintViolation {
                description,
                details
            }
        },
        "GenericError" => {
            OCPP1_6Error::GenericError {
                description,
                details
            }
        },
        _ => {
            OCPP1_6Error::GenericError {
                description,
                details
            }
        }
    };

    let mut lock = message_queue.lock().await;
    if let Some(sender) = lock.remove(&message_id) {
        if let Err(_) = sender.send(Err(err)) {
            warn!("The message had timed out");
        }
    } else {
        warn!("We were not expecting this message, dropping it...")
    }

    Ok(())
}

async fn handle_ocpp_1_6_call(charger: Arc<Mutex<Charger>>, raw_payload: &str, sink: Arc<tokio::sync::Mutex<SplitSink<WebSocketStream, Message>>>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let (_, message_id, action, payload): (i64, String, String, Value) = serde_json::from_str(&raw_payload)?;
    let mut lock = charger.lock().await;

    info!(
        charger_id = lock.id.to_string(),
        protocol = OcppProtocol::Ocpp1_6.to_string(),
        message_id = &message_id,
        action = &action,
        payload = payload.to_string(),
        "Incoming call <--"
    );

    let response: Result<Value, OCPP1_6Error> = match action.as_str() {
        "Authorize" => {
            lock.ocpp1_6().handle_authorize(serde_json::from_value(payload)?).await.map(|i| serde_json::to_value(&i).unwrap())
        }
        "BootNotification" => {
            lock.ocpp1_6().handle_boot_notification(serde_json::from_value(payload)?).await.map(|i| serde_json::to_value(&i).unwrap())
        }
        "DataTransfer" => {
            lock.ocpp1_6().handle_data_transfer(serde_json::from_value(payload)?).await.map(|i| serde_json::to_value(&i).unwrap())
        }
        "DiagnosticsStatusNotification" => {
            lock.ocpp1_6().handle_diagnostics_status_notification(serde_json::from_value(payload)?).await.map(|i| serde_json::to_value(&i).unwrap())
        }
        "FirmwareStatusNotification" => {
            lock.ocpp1_6().handle_firmware_status_notification(serde_json::from_value(payload)?).await.map(|i| serde_json::to_value(&i).unwrap())
        }
        "Heartbeat" => {
            lock.ocpp1_6().handle_heartbeat(serde_json::from_value(payload)?).await.map(|i| serde_json::to_value(&i).unwrap())
        }
        "MeterValues" => {
            lock.ocpp1_6().handle_meter_values(serde_json::from_value(payload)?).await.map(|i| serde_json::to_value(&i).unwrap())
        }
        "StartTransaction" => {
            lock.ocpp1_6().handle_start_transaction(serde_json::from_value(payload)?).await.map(|i| serde_json::to_value(&i).unwrap())
        }
        "StatusNotification" => {
            lock.ocpp1_6().handle_status_notification(serde_json::from_value(payload)?).await.map(|i| serde_json::to_value(&i).unwrap())
        }
        "StopTransaction" => {
            lock.ocpp1_6().handle_stop_transaction(serde_json::from_value(payload)?).await.map(|i| serde_json::to_value(&i).unwrap())
        }
        _ => {
            Err(OCPP1_6Error::new_not_implemented(&format!("Action '{}' is not implemented on this server", action)))
        }
    };

    match response {
        Ok(payload) => {
            info!(
                charger_id = lock.id.to_string(),
                protocol = OcppProtocol::Ocpp1_6.to_string(),
                message_id = &message_id,
                action = &action,
                payload = &payload.to_string(),
                "Responding to call -->"
            );
            {
                let mut sink_lock = sink.lock().await;
                sink_lock.send(Message::Text(serde_json::to_string(&(OCPP_CALL_RESULT, message_id.to_string(), payload))?)).await?;
            }
            let charger = charger.clone();
            tokio::spawn(async move {
                let mut lock = charger.lock().await;
                if let Err(err) = lock.ocpp1_6().post_request(&action).await {
                    warn!(charger_id = lock.id.to_string(), protocol = OcppProtocol::Ocpp1_6.to_string(), message_id = &message_id, action = &action, error_message = err.to_string(), "Failed to handle post hook");
                }
            });

        }
        Err(error) => {
            warn!(
                charger_id = lock.id.to_string(),
                protocol = OcppProtocol::Ocpp1_6.to_string(),
                message_id = &message_id,
                action = &action,
                error_code = error.code(),
                error_description = error.description(),
                "Responding to call with error -->"
            );
            let mut lock = sink.lock().await;
            lock.send(Message::Text(serde_json::to_string(&(OCPP_ERROR, message_id, error.code(), error.description(), error.details()))?)).await?;
        }
    }

    Ok(())
}