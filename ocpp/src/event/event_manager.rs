use crate::event::amqp_event_handler::AmqpEventHandler;
use crate::event::event_handler::EventHandler;
use ocpp_csms_server_sdk::event::EventPayload;
use shared::Config;
use std::sync::Arc;
use tracing::info;

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

    pub async fn send_event(&self, payload: EventPayload) {
        for handler in self.event_handlers.iter() {
            handler.send_event(payload.clone()).await;
        }
    }
}
