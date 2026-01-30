use crate::event::EventPayload;
use std::fmt;

#[async_trait::async_trait]
pub trait EventHandler: Send + Sync + fmt::Debug {
    async fn send_event(&self, payload: EventPayload);
}
