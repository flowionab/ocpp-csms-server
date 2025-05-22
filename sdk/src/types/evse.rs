use crate::ocpp_csms_server;
use crate::types::connector::Connector;

#[derive(Debug, Clone, Default)]
pub struct Evse {
    pub id: String,
    pub ocpp_id: u32,
    pub connectors: Vec<Connector>,
}

impl From<ocpp_csms_server::Evse> for Evse {
    fn from(value: ocpp_csms_server::Evse) -> Self {
        Self {
            id: value.id,
            ocpp_id: value.ocpp_id,
            connectors: value.connectors.into_iter().map(Connector::from).collect(),
        }
    }
}
