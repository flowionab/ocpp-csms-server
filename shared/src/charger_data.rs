use crate::evse_data::EvseData;
use crate::Ocpp1_6Configuration;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Default, FromRow)]
pub struct ChargerData {
    pub id: String,
    pub model: Option<String>,
    pub vendor: Option<String>,
    pub serial_number: Option<String>,
    pub firmware_version: Option<String>,
    pub iccid: Option<String>,
    pub imsi: Option<String>,
    pub evses: Vec<EvseData>,

    pub ocpp1_6configuration: Option<Ocpp1_6Configuration>,
}

impl ChargerData {
    pub fn evse_by_ocpp_id_or_create(&mut self, ocpp_id: u32) -> &mut EvseData {
        if self.evse_by_ocpp_id(ocpp_id).is_some() {
            self.evse_by_ocpp_id_mut(ocpp_id).unwrap()
        } else {
            self.create_evse(ocpp_id)
        }
    }

    fn create_evse(&mut self, ocpp_id: u32) -> &mut EvseData {
        let new_evse = EvseData::new(ocpp_id);
        self.evses.push(new_evse);
        self.evses.last_mut().unwrap()
    }

    pub fn evse_by_ocpp_id(&self, ocpp_id: u32) -> Option<&EvseData> {
        self.evses.iter().find(|evse| evse.ocpp_evse_id == ocpp_id)
    }

    pub fn evse_by_ocpp_id_mut(&mut self, ocpp_id: u32) -> Option<&mut EvseData> {
        self.evses
            .iter_mut()
            .find(|evse| evse.ocpp_evse_id == ocpp_id)
    }
    pub fn evse(&self, evse_id: Uuid) -> Option<&EvseData> {
        self.evses.iter().find(|evse| evse.id == evse_id)
    }

    pub fn evse_mut(&mut self, evse_id: Uuid) -> Option<&mut EvseData> {
        self.evses.iter_mut().find(|evse| evse.id == evse_id)
    }
}
