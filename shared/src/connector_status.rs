use rust_ocpp::v2_0_1::enumerations::connector_status_enum_type::ConnectorStatusEnumType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum ConnectorStatus {
    #[default]
    Available,
    Occupied,
    Reserved,
    Unavailable,
    Faulted,
}

impl From<ConnectorStatusEnumType> for ConnectorStatus {
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
