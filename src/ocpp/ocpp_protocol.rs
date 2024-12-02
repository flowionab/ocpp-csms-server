use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OcppProtocol {
    Ocpp1_6,
    Ocpp2_0_1,
}

impl TryFrom<&str> for OcppProtocol {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ocpp1.6" => Ok(OcppProtocol::Ocpp1_6),
            "ocpp2.0.1" => Ok(OcppProtocol::Ocpp2_0_1),
            _ => Err("Protocol not recognized".into()),
        }
    }
}

impl ToString for OcppProtocol {
    fn to_string(&self) -> String {
        match self {
            OcppProtocol::Ocpp1_6 => "ocpp1.6".to_string(),
            OcppProtocol::Ocpp2_0_1 => "ocpp2.0.1".to_string(),
        }
    }
}
