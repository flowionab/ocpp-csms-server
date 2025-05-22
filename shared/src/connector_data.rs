use crate::{ConnectorStatus, ConnectorType};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectorData {
    pub id: Uuid,
    pub ocpp_id: u32,
    pub connector_type: ConnectorType,
    pub status: ConnectorStatus,
}
