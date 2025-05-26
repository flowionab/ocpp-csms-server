use crate::event::event_handler::EventHandler;
use crate::event::payload::ConnectorStatusEvent;
use crate::event::ConnectorStatus;
use chrono::{DateTime, Utc};
use lapin::{BasicProperties, Connection, ConnectionProperties, ExchangeKind};
use serde::Serialize;
use std::env;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug)]
pub struct AmqpEventHandler {
    connection: Connection,
}

const EVENT_EXCHANGE_NAME: &str = "ocpp_csms_server_events";

impl AmqpEventHandler {
    pub async fn setup() -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        info!("setting up amqp event handler");

        let url = env::var("AMQP_URL").expect("AMQP_URL must be set");

        let connection = Connection::connect(
            &url,
            ConnectionProperties::default()
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
                Default::default(),
                Default::default(),
            )
            .await?;

        info!("declared exchange 'ocpp_csms_server_events'");

        Ok(AmqpEventHandler { connection })
    }

    async fn send<T: Serialize>(
        &self,
        payload: T,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let raw_payload = serde_json::to_string(&payload)?;

        let channel = self.connection.create_channel().await?;

        let properties =
            BasicProperties::default().with_content_encoding("application/json".to_string().into());

        channel
            .basic_publish(
                EVENT_EXCHANGE_NAME,
                "",
                Default::default(),
                raw_payload.as_bytes(),
                properties,
            )
            .await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl EventHandler for AmqpEventHandler {
    async fn send_connector_status_event(
        &self,
        charger_id: String,
        status: ConnectorStatus,
        timestamp: DateTime<Utc>,
        evse_id: Uuid,
        connector_id: Uuid,
    ) {
        let payload = ConnectorStatusEvent {
            charger_id,
            connector_id,
            evse_id,
            status,
            timestamp,
        };

        if let Err(error) = self.send(payload).await {
            error!(
                error_message = error.to_string(),
                "error sending connector status event"
            );
        }
    }
}
