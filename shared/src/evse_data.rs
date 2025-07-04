use crate::connector_data::ConnectorData;
use crate::phase_metric::PhaseMetric;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvseData {
    pub id: Uuid,
    pub ocpp_evse_id: u32,
    pub connectors: Vec<ConnectorData>,
    pub watt_output: PhaseMetric<f32>,
    pub ampere_output: PhaseMetric<f32>,
    pub voltage: PhaseMetric<f32>,
}

impl EvseData {
    pub fn new(ocpp_evse_id: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            ocpp_evse_id,
            connectors: vec![ConnectorData::new(1)],
            watt_output: PhaseMetric::default(),
            ampere_output: PhaseMetric::default(),
            voltage: PhaseMetric::default(),
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
