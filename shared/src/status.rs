use rust_ocpp::v2_0_1::enumerations::connector_status_enum_type::ConnectorStatusEnumType;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display)]
pub enum Status {
    Available,
    Occupied,
    Reserved,
    Unavailable,
    Faulted,
}

impl Default for Status {
    fn default() -> Self {
        Self::Available
    }
}

impl From<ConnectorStatusEnumType> for Status {
    fn from(status: ConnectorStatusEnumType) -> Self {
        match status {
            ConnectorStatusEnumType::Available => Self::Available,
            ConnectorStatusEnumType::Occupied => Self::Occupied,
            ConnectorStatusEnumType::Reserved => Self::Reserved,
            ConnectorStatusEnumType::Unavailable => Self::Unavailable,
            ConnectorStatusEnumType::Faulted => Self::Faulted,
        }
    }
}
