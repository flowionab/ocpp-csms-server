#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ChargerModel {
    Unknown,
}

impl ChargerModel {
    #[allow(dead_code)]
    pub fn from_vendor_and_model(_vendor: &str, _model: &str) -> Self {
        Self::Unknown
    }
}
