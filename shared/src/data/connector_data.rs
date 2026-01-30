use crate::data::{ConnectorStatus, ConnectorType};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorData {
    pub id: Uuid,
    pub ocpp_id: u32,
    pub connector_type: ConnectorType,
    pub status: ConnectorStatus,
}

impl ConnectorData {
    pub fn new(ocpp_id: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            ocpp_id,
            connector_type: ConnectorType::Undetermined,
            status: ConnectorStatus::Available,
        }
    }
}
