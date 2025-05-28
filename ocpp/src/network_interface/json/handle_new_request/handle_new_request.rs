use crate::charger::ChargerPool;
use crate::network_interface::charger_factory::ChargerFactory;
use crate::network_interface::json::authentication_handler::AuthenticationHandler;
use crate::network_interface::json::extract_password::extract_password;
use crate::network_interface::json::handle_new_request::handle_new_websocket_connection::handle_new_websocket_connection;
use crate::network_interface::json::ocpp_network_interface_handle::OcppNetworkInterfaceHandle;
use crate::network_interface::json::validate_protocol::validate_protocol;
use crate::network_interface::ocpp1_6_request_receiver::Ocpp16RequestReceiver;
use crate::network_interface::ocpp2_0_1_request_receiver::Ocpp2_0_1RequestReceiver;
use crate::network_interface::protocol_handle::ProtocolHandle;
use crate::network_interface::{
    Ocpp16NetworkInterfaceHandle, Ocpp2_0_1NetworkInterfaceHandle, OcppProtocol,
};
use poem::http::{HeaderMap, HeaderValue};
use poem::web::websocket::WebSocket;
use poem::{IntoResponse, Response};
use shared::Config;
use std::sync::Arc;
use tonic::codegen::http::StatusCode;
use tracing::{info, warn};

pub async fn handle_new_request<
    T: AuthenticationHandler
        + Ocpp16RequestReceiver
        + Ocpp2_0_1RequestReceiver
        + Send
        + Sync
        + 'static,
>(
    config: &Arc<Config>,
    charger_pool: &ChargerPool,
    ws: WebSocket,
    headers: &HeaderMap,
    id: String,
    charger_factory: &Arc<dyn ChargerFactory<T> + Send + Sync>,
) -> Result<Response, Response> {
    let password = extract_password(headers)?;
    let protocol = validate_protocol(headers)?;

    if charger_pool.get(&id).await.is_some() {
        warn!(charger_id = id, "charger already connected");
        return Err(Response::builder()
            .status(StatusCode::CONFLICT)
            .body(format!("Charger with ID {} already connected", id)));
    }

    let handle = Arc::new(OcppNetworkInterfaceHandle::new(Arc::clone(config), &id));

    let mut charger = charger_factory
        .create_charger(
            &id,
            match protocol {
                OcppProtocol::Ocpp1_6 => ProtocolHandle::Ocpp1_6(
                    Arc::clone(&handle) as Arc<dyn Ocpp16NetworkInterfaceHandle + Send + Sync>
                ),
                OcppProtocol::Ocpp2_0_1 => {
                    ProtocolHandle::Ocpp2_0_1(Arc::clone(&handle)
                        as Arc<dyn Ocpp2_0_1NetworkInterfaceHandle + Send + Sync>)
                }
            },
        )
        .await
        .map_err(|err| {
            warn!("Failed to create charger: {}", err);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("Failed to create charger: {}", err))
        })?;

    charger.authenticate_with_password(&password).await?;

    info!(
        charger_id = &id,
        ocpp_protocol = protocol.to_string(),
        "selected a protocol for the connection",
    );

    let charger_factory = Arc::clone(charger_factory);
    let config = Arc::clone(config);
    let mut response = ws
        .protocols(vec!["ocpp1.6", "ocpp2.0.1"])
        .on_upgrade(move |socket| async move {
            handle_new_websocket_connection(
                Arc::clone(&config),
                &id,
                protocol,
                charger,
                &charger_factory,
                &handle,
                socket,
            )
            .await;
        })
        .into_response();

    response.headers_mut().insert(
        "Sec-WebSocket-Protocol",
        HeaderValue::from_str(&protocol.to_string()).unwrap(),
    );

    Ok(response)
}
