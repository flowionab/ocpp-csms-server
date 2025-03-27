use crate::connector_data::ConnectorData;
use crate::Status;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EvseData {
    pub id: Uuid,
    pub ocpp_evse_id: u32,
    pub status: Option<Status>,
    pub connectors: Vec<ConnectorData>,
}

impl EvseData {
    pub fn connector_by_ocpp_id(&self, ocpp_id: u32) -> Option<&ConnectorData> {
        self.connectors
            .iter()
            .find(|connector| connector.ocpp_connector_id == ocpp_id)
    }

    pub fn connector_by_ocpp_id_mut(&mut self, ocpp_id: u32) -> Option<&mut ConnectorData> {
        self.connectors
            .iter_mut()
            .find(|connector| connector.ocpp_connector_id == ocpp_id)
    }
}
