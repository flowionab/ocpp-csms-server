#[allow(dead_code)]
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum ChargerModel {
    Unknown,
    Easee(EaseeChargerModel),
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum EaseeChargerModel {
    Unknown,
    ChargeLite,
    Core,
    One,
    Up,
    Max,
    Home,
}

impl ChargerModel {
    #[allow(dead_code)]
    pub fn from_vendor_and_model(vendor: &str, model: &str) -> Self {
        match vendor {
            "Easee" => Self::Easee(EaseeChargerModel::from_model(model)),
            _ => Self::Unknown,
        }
    }
}

impl EaseeChargerModel {
    #[allow(dead_code)]
    pub fn from_model(model: &str) -> Self {
        match model {
            "Charge Lite" => Self::ChargeLite,
            "Core" => Self::Core,
            "One" => Self::One,
            "Up" => Self::Up,
            "Max" => Self::Max,
            "Home" => Self::Home,
            _ => Self::Unknown,
        }
    }
}
