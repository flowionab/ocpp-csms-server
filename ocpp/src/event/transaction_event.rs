use crate::event::evse_info::EvseInfo;
use crate::event::meter_value::MeterValue;
use crate::event::transaction_event_trigger_reason::TransactionEventTriggerReason;
use crate::event::transaction_event_type::TransactionEventType;
use crate::event::transaction_info::TransactionInfo;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEvent {
    pub charger_id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: TransactionEventType,
    pub trigger_reason: TransactionEventTriggerReason,
    pub number_of_phases_used: Option<i32>,
    pub cable_max_current: Option<i32>,
    pub reservation_id: Option<i32>,
    pub transaction_info: TransactionInfo,
    pub evse: EvseInfo,
    pub meter_values: Vec<MeterValue>,
}
