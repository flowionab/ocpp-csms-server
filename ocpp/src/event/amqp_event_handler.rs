use crate::event::event_handler::EventHandler;
use lapin::options::ExchangeDeclareOptions;
use lapin::types::LongString;
use lapin::{BasicProperties, Connection, ConnectionProperties, ExchangeKind};
use ocpp_csms_server_sdk::event::EventPayload;
use std::collections::BTreeMap;
use std::env;
use tracing::{debug, error, info};

#[derive(Debug)]
pub struct AmqpEventHandler {
    connection: Connection,
}

const EVENT_EXCHANGE_NAME: &str = "ocpp_csms_server_events";

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl AmqpEventHandler {
    pub async fn setup() -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        info!("setting up amqp event handler");

        let url = env::var("AMQP_URL").expect("AMQP_URL must be set");

        let connection = Connection::connect(
            &url,
            ConnectionProperties::default()
                .with_connection_name(format!("ocpp_csms_server:{}", VERSION).into())
                .with_executor(tokio_executor_trait::Tokio::current())
                .with_reactor(tokio_reactor_trait::Tokio),
        )
        .await?;

        info!("connected to amqp server");

        let channel = connection.create_channel().await?;

        channel
            .exchange_declare(
                EVENT_EXCHANGE_NAME,
                ExchangeKind::Fanout,
                ExchangeDeclareOptions {
                    passive: false,
                    durable: true,
                    auto_delete: false,
                    internal: false,
                    nowait: false,
                },
                Default::default(),
            )
            .await?;

        info!("declared exchange 'ocpp_csms_server_events'");

        channel.close(0, "").await?;

        Ok(AmqpEventHandler { connection })
    }

    async fn send(
        &self,
        payload: EventPayload,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let raw_payload = serde_json::to_string(&payload)?;

        let channel = self.connection.create_channel().await?;

        let mut headers = BTreeMap::new();
        headers.insert(
            "chargerId".into(),
            LongString::from(payload.charger_id()).into(),
        );

        let properties = BasicProperties::default()
            .with_content_type("application/json".into())
            .with_content_encoding("utf-8".into())
            .with_timestamp(payload.timestamp().timestamp() as u64)
            .with_headers(headers.into());

        let confirm = channel
            .basic_publish(
                EVENT_EXCHANGE_NAME,
                "",
                Default::default(),
                raw_payload.as_bytes(),
                properties,
            )
            .await?;

        confirm.await?;

        debug!(payload = raw_payload, "sent event to AMQP exchange");

        channel.close(0, "").await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl EventHandler for AmqpEventHandler {
    async fn send_event(&self, payload: EventPayload) {
        if let Err(error) = self.send(payload).await {
            error!(
                error_message = error.to_string(),
                "error sending connector status event"
            );
        }
    }
}
