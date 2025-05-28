use crate::charger::ChargerPool;
use crate::network_interface::charger_factory::ChargerFactory;
use crate::network_interface::json::authentication_handler::AuthenticationHandler;
use crate::network_interface::json::handle_new_request::handle_new_request;
use crate::network_interface::ocpp1_6_request_receiver::Ocpp16RequestReceiver;
use crate::network_interface::ocpp2_0_1_request_receiver::Ocpp2_0_1RequestReceiver;
use poem::http::HeaderMap;
use poem::web::websocket::WebSocket;
use poem::web::{Data, Path};
use poem::{handler, Response};
use shared::Config;
use std::sync::Arc;
use tracing::instrument;

type DataType<'a, T> = Data<&'a (
    Arc<Config>,
    ChargerPool,
    Arc<dyn ChargerFactory<T> + Send + Sync>,
)>;

#[instrument(skip_all)]
#[handler]
pub async fn ocpp_handler<
    T: AuthenticationHandler
        + Ocpp16RequestReceiver
        + Ocpp2_0_1RequestReceiver
        + Send
        + Sync
        + 'static,
>(
    ws: WebSocket,
    headers: &HeaderMap,
    data: DataType<'_, T>,
    Path(id): Path<String>,
) -> Response {
    handle_new_request::<T>(&data.0 .0, &data.0 .1, ws, headers, id, &data.0 .2)
        .await
        .unwrap_or_else(|r| r)
}
