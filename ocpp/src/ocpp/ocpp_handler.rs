use crate::charger::{Charger, ChargerPool};
use crate::ocpp::extract_password::extract_password;
use crate::ocpp::handle_message::handle_message;
use crate::ocpp::validate_protocol::validate_protocol;
use futures::{StreamExt, TryStreamExt};
use poem::http::{HeaderMap, HeaderValue};
use poem::web::websocket::WebSocket;
use poem::web::{Data, Path};
use poem::{handler, IntoResponse, Response};
use shared::{Config, DataStore};
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, instrument};

#[instrument(skip_all)]
#[handler]
pub async fn ocpp_handler(
    ws: WebSocket,
    headers: &HeaderMap,
    data: Data<&(Config, Arc<dyn DataStore>, ChargerPool)>,
    Path(id): Path<String>,
) -> Response {
    handle(
        data.0 .0.clone(),
        Arc::clone(&data.0 .1),
        data.0 .2.clone(),
        ws,
        headers,
        id,
    )
    .await
    .unwrap_or_else(|r| r)
}

//#[instrument]
async fn handle(
    config: Config,
    data_store: Arc<dyn DataStore>,
    charger_pool: ChargerPool,
    ws: WebSocket,
    headers: &HeaderMap,
    id: String,
) -> Result<Response, Response> {
    info!(charger_id = &id, "Got connection from charger");
    let ocpp1_6message_queue = Arc::new(Mutex::new(BTreeMap::new()));
    let ocpp2_0_1message_queue = Arc::new(Mutex::new(BTreeMap::new()));
    let mut charger =
        Charger::setup(&id, &config, data_store, Arc::clone(&ocpp1_6message_queue)).await?;

    if config
        .ocpp
        .unwrap_or_default()
        .disable_charger_auth
        .unwrap_or_default()
        == false
    {
        let password = extract_password(headers)?;
        charger.authenticate_with_password(password).await?;
    }

    let charger = Arc::new(Mutex::new(charger));
    charger_pool.insert(&id, &charger).await;

    let protocol = validate_protocol(headers)?;

    info!(
        charger_id = &id,
        ocpp_protocol = protocol.to_string(),
        "Selected a protocol for the connection",
    );

    let mut response = ws
        .protocols(vec!["ocpp1.6", "ocpp2.0.1"])
        .on_upgrade(move |socket| async move {
            info!(
                charger_id = &id,
                ocpp_protocol = protocol.to_string(),
                "Websocket connection established",
            );
            let (sink, stream) = socket.split();

            let sink = Arc::new(Mutex::new(sink));
            {
                let mut lock = charger.lock().await;
                lock.attach_sink(Arc::clone(&sink));
                lock.set_protocol(protocol);
            }

            if let Err(err) = stream
                .map_err(Into::<Box<dyn std::error::Error + Send + Sync>>::into)
                .try_for_each_concurrent(None, |message| {
                    let charger = charger.clone();
                    let sink = Arc::clone(&sink);
                    let ocpp1_6message_queue = Arc::clone(&ocpp1_6message_queue);
                    let ocpp2_0_1message_queue = Arc::clone(&ocpp2_0_1message_queue);
                    async move {
                        handle_message(
                            charger,
                            message,
                            protocol,
                            sink,
                            ocpp1_6message_queue,
                            ocpp2_0_1message_queue,
                        )
                        .await?;
                        Ok(())
                    }
                })
                .await
            {
                info!(
                    charger_id = &id,
                    ocpp_protocol = protocol.to_string(),
                    error_message = err.to_string(),
                    "Connection closed with error"
                )
            } else {
                info!(
                    charger_id = &id,
                    ocpp_protocol = protocol.to_string(),
                    "Connection closed"
                )
            }
        })
        .into_response();

    response.headers_mut().insert(
        "Sec-WebSocket-Protocol",
        HeaderValue::from_str(&protocol.to_string()).unwrap(),
    );

    Ok(response)
}
