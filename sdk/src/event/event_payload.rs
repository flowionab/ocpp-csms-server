use crate::event::transaction_event::TransactionEvent;
use crate::event::transaction_stopped_event::TransactionStoppedEvent;
use crate::event::TransactionStartedEvent;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum EventPayload {
    TransactionEvent(TransactionEvent),
    TransactionStartedEvent(TransactionStartedEvent),
    TransactionStoppedEvent(TransactionStoppedEvent),
}

impl EventPayload {
    pub fn timestamp(&self) -> &DateTime<Utc> {
        match self {
            EventPayload::TransactionEvent(event) => &event.timestamp,
            EventPayload::TransactionStartedEvent(event) => &event.started_at,
            EventPayload::TransactionStoppedEvent(event) => &event.stopped_at,
        }
    }

    pub fn charger_id(&self) -> &str {
        match self {
            EventPayload::TransactionEvent(event) => &event.charger_id,
            EventPayload::TransactionStartedEvent(event) => &event.charger_id,
            EventPayload::TransactionStoppedEvent(event) => &event.charger_id,
        }
    }
}
