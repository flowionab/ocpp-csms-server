use crate::event::EventPayload;
use crate::event::event_handler::EventHandler;
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

        if event_handlers.is_empty() {
            info!("no event handlers configured");
        }

        Ok(EventManager {
            event_handlers: Arc::new(event_handlers),
        })
    }

    pub async fn send_event(&self, payload: EventPayload) {
        unimplemented!()
    }
}
