use crate::event::transaction_event::TransactionEvent;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum EventPayload {
    TransactionEvent(TransactionEvent),
}

impl EventPayload {
    pub fn timestamp(&self) -> &DateTime<Utc> {
        match self {
            EventPayload::TransactionEvent(event) => &event.timestamp,
        }
    }

    pub fn charger_id(&self) -> &str {
        match self {
            EventPayload::TransactionEvent(event) => &event.charger_id,
        }
    }
}
