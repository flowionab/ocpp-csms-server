use crate::ocpp_csms_server;
use crate::types::{ConnectorStatus, ConnectorType};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct Connector {
    pub id: Uuid,
    pub charger_id: String,
    pub evse_id: Uuid,
    pub ocpp_id: u32,
    pub connector_type: ConnectorType,
    pub status: ConnectorStatus,
}

impl TryFrom<ocpp_csms_server::Connector> for Connector {
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
    fn try_from(value: ocpp_csms_server::Connector) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::from_str(&value.id)?,
            charger_id: value.charger_id,
            evse_id: Uuid::from_str(&value.evse_id)?,
            ocpp_id: value.ocpp_id,
            connector_type: ocpp_csms_server::ConnectorType::try_from(value.r#type)
                .unwrap_or_default()
                .into(),
            status: ocpp_csms_server::ConnectorStatus::try_from(value.status)
                .unwrap_or_default()
                .into(),
        })
    }
}
