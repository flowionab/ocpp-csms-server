use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionStoppedEvent {
    pub charger_id: String,
    pub evse_id: Uuid,
    pub connector_id: Uuid,
    pub transaction_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub stopped_at: DateTime<Utc>,
}
