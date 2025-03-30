use crate::event::ConnectorStatus;
use chrono::{DateTime, Utc};
use std::fmt;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait EventHandler: Send + Sync + fmt::Debug {
    async fn send_connector_status_event(
        &self,
        charger_id: String,
        status: ConnectorStatus,
        timestamp: DateTime<Utc>,
        evse_id: Uuid,
        connector_id: Uuid,
    );
}
