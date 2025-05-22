#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum ConnectorStatus {
    #[default]
    Available,
    Occupied,
    Reserved,
    Unavailable,
    Faulted,
}

impl From<crate::ocpp_csms_server::ConnectorStatus> for ConnectorStatus {
    fn from(status: crate::ocpp_csms_server::ConnectorStatus) -> Self {
        match status {
            crate::ocpp_csms_server::ConnectorStatus::Available => Self::Available,
            crate::ocpp_csms_server::ConnectorStatus::Occupied => Self::Occupied,
            crate::ocpp_csms_server::ConnectorStatus::Reserved => Self::Reserved,
            crate::ocpp_csms_server::ConnectorStatus::Unavailable => Self::Unavailable,
            crate::ocpp_csms_server::ConnectorStatus::Faulted => Self::Faulted,
            crate::ocpp_csms_server::ConnectorStatus::Unspecified => Self::Available,
        }
    }
}
