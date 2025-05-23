#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum ConnectorStatus {
    #[default]
    Available,
    Occupied,
    Reserved,
    Unavailable,
    Faulted,
}

impl ConnectorStatus {
    pub fn is_available(&self) -> bool {
        matches!(self, ConnectorStatus::Available)
    }

    pub fn is_occupied(&self) -> bool {
        matches!(self, ConnectorStatus::Occupied)
    }

    pub fn is_reserved(&self) -> bool {
        matches!(self, ConnectorStatus::Reserved)
    }

    pub fn is_unavailable(&self) -> bool {
        matches!(self, ConnectorStatus::Unavailable)
    }

    pub fn is_faulted(&self) -> bool {
        matches!(self, ConnectorStatus::Faulted)
    }
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
