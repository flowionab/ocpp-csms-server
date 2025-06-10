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

impl From<rust_ocpp::v1_6::types::ChargePointStatus> for ConnectorStatus {
    fn from(status: rust_ocpp::v1_6::types::ChargePointStatus) -> Self {
        match status {
            rust_ocpp::v1_6::types::ChargePointStatus::Available => Self::Available,
            rust_ocpp::v1_6::types::ChargePointStatus::Preparing => Self::Occupied,
            rust_ocpp::v1_6::types::ChargePointStatus::Charging => Self::Occupied,
            rust_ocpp::v1_6::types::ChargePointStatus::SuspendedEVSE => Self::Occupied,
            rust_ocpp::v1_6::types::ChargePointStatus::SuspendedEV => Self::Occupied,
            rust_ocpp::v1_6::types::ChargePointStatus::Finishing => Self::Occupied,
            rust_ocpp::v1_6::types::ChargePointStatus::Reserved => Self::Reserved,
            rust_ocpp::v1_6::types::ChargePointStatus::Unavailable => Self::Unavailable,
            rust_ocpp::v1_6::types::ChargePointStatus::Faulted => Self::Faulted,
        }
    }
}
