

#[derive(Debug, Clone)]
pub enum ChargerModel {
    Unknown(String)
}

impl ChargerModel {
    pub fn from_vendor_and_model(vendor: &str, model: &str) -> Self {
        match vendor {
            _ => {
                Self::Unknown(format!("{}:{}", vendor, model))
            }
        }
    }
}

