use std::fmt::Error;
use std::sync::Arc;
use futures::{StreamExt, TryStreamExt};
use poem::{handler, head, IntoResponse, Response};
use poem::http::{HeaderMap, HeaderValue};
use poem::web::{Data, Path};
use poem::web::websocket::WebSocket;
use tokio::sync::Mutex;
use tracing::info;
use crate::charger::Charger;
use crate::ocpp::handle_message::handle_message;
use crate::ocpp::validate_protocol::validate_protocol;

#[handler]
pub async fn ocpp_handler(ws: WebSocket, headers: &HeaderMap, Path((id)): Path<(String)>,) -> Response {
    handle(ws, headers, id).await.unwrap_or_else(|r| r)
}

async fn handle(ws: WebSocket, headers: &HeaderMap, id: String) -> Result<Response, Response> {
    info!(
        charger_id = &id,
        "Got connection from charger"
    );
    // We should probably validate the auth header here
    let charger = Arc::new(Mutex::new(Charger::new(&id)));

    let protocol = validate_protocol(headers)?;

    info!(
        charger_id = &id,
        ocpp_protocol = protocol.to_string(),
        "Selected a protocol for the connection",
    );


    let mut response = ws.protocols(vec!["ocpp1.6", "ocpp2.0.1"])
        .on_upgrade(move |socket| async move {
            info!(
                charger_id = &id,
                ocpp_protocol = protocol.to_string(),
                "Websocket connection established",
            );
            let (sink, stream) = socket.split();

            let sink  = Arc::new(Mutex::new(sink));

            if let Err(err) = stream
                .map_err(Into::<Box<dyn std::error::Error + Send + Sync>>::into)
                .try_for_each(|message|{
                    let charger = charger.clone();
                    let sink = Arc::clone(&sink);
                async move {
                handle_message(charger, message, protocol, sink).await?;
                Ok(())
            }}).await {
                info!(charger_id = &id,
                ocpp_protocol = protocol.to_string(),
                error_message = err.to_string(),
                "Connection closed with error")
            } else {
                info!(charger_id = &id,
                ocpp_protocol = protocol.to_string(),
                "Connection closed")
            }
        }).into_response();

    response.headers_mut().insert(
        "Sec-WebSocket-Protocol",
        HeaderValue::from_str(&protocol.to_string()).unwrap(),
    );

    Ok(response)
}