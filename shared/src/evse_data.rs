use crate::connector_data::ConnectorData;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvseData {
    pub id: Uuid,
    pub ocpp_evse_id: u32,
    pub connectors: Vec<ConnectorData>,
}

impl EvseData {
    pub fn new(ocpp_evse_id: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            ocpp_evse_id,
            connectors: vec![ConnectorData::new(1)],
        }
    }

    pub fn connector_by_ocpp_id(&self, ocpp_id: u32) -> Option<&ConnectorData> {
        self.connectors
            .iter()
            .find(|connector| connector.ocpp_id == ocpp_id)
    }

    pub fn connector_by_ocpp_id_mut(&mut self, ocpp_id: u32) -> Option<&mut ConnectorData> {
        self.connectors
            .iter_mut()
            .find(|connector| connector.ocpp_id == ocpp_id)
    }
}
