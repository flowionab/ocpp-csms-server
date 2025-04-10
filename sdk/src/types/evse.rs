use crate::ocpp_csms_server;

#[derive(Debug, Clone, Default)]
pub struct Evse {
    pub id: String,
    pub ocpp_connector_id: u32,
    pub status: Option<String>,
}

impl From<ocpp_csms_server::Evse> for Evse {
    fn from(value: ocpp_csms_server::Evse) -> Self {
        Self {
            id: value.id,
            ocpp_connector_id: value.ocpp_connector_id,
            status: value.status,
        }
    }
}
