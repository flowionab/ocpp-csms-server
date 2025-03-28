use crate::evse_data::EvseData;
use crate::status::Status;
use crate::Ocpp1_6Configuration;
use sqlx::FromRow;

#[derive(Debug, Clone, Default, FromRow)]
pub struct ChargerData {
    pub id: String,
    pub model: Option<String>,
    pub vendor: Option<String>,
    pub serial_number: Option<String>,
    pub firmware_version: Option<String>,
    pub iccid: Option<String>,
    pub imsi: Option<String>,
    pub status: Option<Status>,
    pub evses: Vec<EvseData>,

    pub ocpp1_6configuration: Option<Ocpp1_6Configuration>,
}

impl ChargerData {
    pub fn evse_by_ocpp_id(&self, ocpp_id: u32) -> Option<&EvseData> {
        self.evses.iter().find(|evse| evse.ocpp_evse_id == ocpp_id)
    }

    pub fn evse_by_ocpp_id_mut(&mut self, ocpp_id: u32) -> Option<&mut EvseData> {
        self.evses
            .iter_mut()
            .find(|evse| evse.ocpp_evse_id == ocpp_id)
    }
}
