use sqlx::FromRow;
use crate::charger::charger_model::ChargerModel;

#[derive(Debug, Clone, Default, FromRow)]
pub struct ChargerData {
    pub id: String,
    pub model: Option<String>,
    pub vendor: Option<String>,
    pub serial_number: Option<String>,
    pub firmware_version: Option<String>,
    pub iccid: Option<String>,
    pub imsi: Option<String>,
}