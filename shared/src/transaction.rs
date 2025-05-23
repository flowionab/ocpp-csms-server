use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Transaction {
    pub id: Uuid,
    pub charger_id: String,
    pub ocpp_transaction_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub watt_charged: i32,
    pub is_authorized: bool,
}
