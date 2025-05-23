use crate::ocpp_csms_server;
use crate::types::{ConnectorStatus, ConnectorType};

#[derive(Debug, Clone, Default)]
pub struct Connector {
    pub id: String,
    pub ocpp_id: u32,
    pub connector_type: ConnectorType,
    pub status: ConnectorStatus,
}

impl From<ocpp_csms_server::Connector> for Connector {
    fn from(value: ocpp_csms_server::Connector) -> Self {
        Self {
            id: value.id,
            ocpp_id: value.ocpp_id,
            connector_type: ocpp_csms_server::ConnectorType::try_from(value.r#type)
                .unwrap_or_default()
                .into(),
            status: ocpp_csms_server::ConnectorStatus::try_from(value.status)
                .unwrap_or_default()
                .into(),
        }
    }
}
