use crate::event::payload::boot_reason::BootReason;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, serde::Serialize)]
pub struct BootedEvent {
    pub charger_id: String,
    pub reason: BootReason,
    pub timestamp: DateTime<Utc>,
}
