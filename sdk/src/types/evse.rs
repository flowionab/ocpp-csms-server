use crate::ocpp_csms_server;
use crate::types::connector::Connector;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct Evse {
    pub id: Uuid,
    pub charger_id: String,
    pub ocpp_id: u32,
    pub connectors: Vec<Connector>,
}

impl TryFrom<ocpp_csms_server::Evse> for Evse {
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
    fn try_from(value: ocpp_csms_server::Evse) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::from_str(&value.id)?,
            charger_id: value.charger_id,
            ocpp_id: value.ocpp_id,
            connectors: value
                .connectors
                .into_iter()
                .map(Connector::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
