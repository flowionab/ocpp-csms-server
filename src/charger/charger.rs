use crate::charger::charger_model::ChargerModel;
use crate::charger::charger_model::ChargerModel::Unknown;
use crate::charger::ocpp1_6interface::Ocpp1_6Interface;

#[derive(Debug, Clone)]
pub struct Charger {
    pub id: String,
    pub authenticated: bool,
    pub model: Option<ChargerModel>,
    pub vendor: Option<String>,
}

impl Charger {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            authenticated: false,
            model: None,
            vendor: None
        }
    }

    pub fn ocpp1_6(&mut self) -> Ocpp1_6Interface {
        Ocpp1_6Interface::new(self)
    }
}