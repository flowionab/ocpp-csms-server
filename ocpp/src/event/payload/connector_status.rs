use rust_ocpp::v1_6::types::ChargePointStatus;
use rust_ocpp::v2_0_1::enumerations::connector_status_enum_type::ConnectorStatusEnumType;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default, Copy)]
pub enum ConnectorStatus {
    #[default]
    Available,
    Occupied,
    Faulted,
    Unavailable,
    Reserved,
}

impl From<ConnectorStatusEnumType> for ConnectorStatus {
    fn from(status: ConnectorStatusEnumType) -> Self {
        match status {
            ConnectorStatusEnumType::Available => ConnectorStatus::Available,
            ConnectorStatusEnumType::Occupied => ConnectorStatus::Occupied,
            ConnectorStatusEnumType::Reserved => ConnectorStatus::Reserved,
            ConnectorStatusEnumType::Unavailable => ConnectorStatus::Unavailable,
            ConnectorStatusEnumType::Faulted => ConnectorStatus::Faulted,
        }
    }
}

impl From<ChargePointStatus> for ConnectorStatus {
    fn from(status: ChargePointStatus) -> Self {
        match status {
            ChargePointStatus::Available => Self::Available,
            ChargePointStatus::Preparing => Self::Occupied,
            ChargePointStatus::Charging => Self::Occupied,
            ChargePointStatus::SuspendedEVSE => Self::Occupied,
            ChargePointStatus::SuspendedEV => Self::Occupied,
            ChargePointStatus::Finishing => Self::Occupied,
            ChargePointStatus::Reserved => Self::Reserved,
            ChargePointStatus::Unavailable => Self::Unavailable,
            ChargePointStatus::Faulted => Self::Faulted,
        }
    }
}
