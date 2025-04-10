use crate::ocpp_csms_server;

pub struct ChargerSummary {
    pub id: String,
    pub serial_number: Option<String>,
    pub model: Option<String>,
    pub vendor: Option<String>,
}

impl From<ocpp_csms_server::ChargerSummary> for ChargerSummary {
    fn from(value: ocpp_csms_server::ChargerSummary) -> Self {
        Self {
            id: value.id,
            serial_number: value.serial_number,
            model: value.model,
            vendor: value.vendor,
        }
    }
}
