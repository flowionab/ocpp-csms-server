use crate::event::charging_state::ChargingState;
use crate::event::stopped_reason::StoppedReason;
use chrono::Duration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInfo {
    pub id: Uuid,
    pub charging_state: Option<ChargingState>,
    pub time_spent_charging: Option<Duration>,
    pub stopped_reason: Option<StoppedReason>,
}
