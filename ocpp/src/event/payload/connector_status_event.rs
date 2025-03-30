use crate::event::ConnectorStatus;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ConnectorStatusEvent {
    pub charger_id: String,
    pub evse_id: Uuid,
    pub connector_id: Uuid,
    pub status: ConnectorStatus,
    pub timestamp: DateTime<Utc>,
}
