use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, strum::Display)]
pub enum OcppProtocol {
    #[strum(to_string = "ocpp1.6")]
    Ocpp1_6,

    #[strum(to_string = "ocpp2.0.1")]
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
