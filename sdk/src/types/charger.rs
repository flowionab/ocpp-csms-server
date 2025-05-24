use crate::ocpp_csms_server;
use crate::types::Evse;

#[derive(Debug, Clone, Default)]
pub struct Charger {
    pub id: String,
    pub serial_number: Option<String>,
    pub model: Option<String>,
    pub vendor: Option<String>,
    pub firmware_version: Option<String>,
    pub iccid: Option<String>,
    pub imsi: Option<String>,
    // pub ocpp1_6_configuration_values: Vec<Ocpp16configuration>,
    pub evses: Vec<Evse>,
    pub is_online: bool,
    pub last_seen: String,
}

impl TryFrom<ocpp_csms_server::Charger> for Charger {
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
    fn try_from(value: ocpp_csms_server::Charger) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            serial_number: value.serial_number,
            model: value.model,
            vendor: value.vendor,
            firmware_version: value.firmware_version,
            iccid: value.iccid,
            imsi: value.imsi,
            // ocpp1_6_configuration_values: value.ocpp1_6_configuration_values,
            evses: value
                .evses
                .into_iter()
                .map(Evse::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            is_online: value.is_online,
            last_seen: value.last_seen,
        })
    }
}
