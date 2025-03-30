use crate::event::amqp_event_handler::AmqpEventHandler;
use crate::event::event_handler::EventHandler;
use crate::event::ConnectorStatus;
use chrono::{DateTime, Utc};
use shared::Config;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct EventManager {
    event_handlers: Arc<Vec<Box<dyn EventHandler>>>,
}

impl EventManager {
    pub async fn from_config(
        config: &Config,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        info!("setting up event listeners");
        let mut event_handlers = Vec::new();

        if let Some(amqp) = &config.amqp {
            if amqp.enabled {
                event_handlers
                    .push(Box::new(AmqpEventHandler::setup().await?) as Box<dyn EventHandler>);
            }
        }

        if event_handlers.is_empty() {
            info!("no event handlers configured");
        }

        Ok(EventManager {
            event_handlers: Arc::new(event_handlers),
        })
    }

    pub async fn send_connector_status_event(
        &self,
        charger_id: String,
        status: ConnectorStatus,
        timestamp: DateTime<Utc>,
        evse_id: Uuid,
        connector_id: Uuid,
    ) {
        for handler in self.event_handlers.iter() {
            handler
                .send_connector_status_event(
                    charger_id.clone(),
                    status,
                    timestamp,
                    evse_id,
                    connector_id,
                )
                .await;
        }
    }
}
