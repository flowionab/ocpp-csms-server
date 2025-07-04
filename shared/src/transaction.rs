use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub charger_id: String,
    pub evse_id: Uuid,
    pub ocpp_transaction_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    // Used for OCPP 1.6
    pub energy_meter_at_start: Option<i32>,
    pub watt_charged: i32,
    pub is_authorized: bool,
}
