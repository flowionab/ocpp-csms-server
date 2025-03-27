use crate::Status;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectorData {
    pub id: Uuid,
    pub ocpp_connector_id: u32,
    pub status: Option<Status>,
}
