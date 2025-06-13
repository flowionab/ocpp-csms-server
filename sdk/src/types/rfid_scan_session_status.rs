use crate::ocpp_csms_server;

pub enum RfidScanSessionStatus {
    Active,
    Completed,
    Failed,
}

impl From<ocpp_csms_server::RfidScanSessionStatus> for RfidScanSessionStatus {
    fn from(status: ocpp_csms_server::RfidScanSessionStatus) -> Self {
        match status {
            ocpp_csms_server::RfidScanSessionStatus::Active => RfidScanSessionStatus::Active,
            ocpp_csms_server::RfidScanSessionStatus::Completed => RfidScanSessionStatus::Completed,
            ocpp_csms_server::RfidScanSessionStatus::Failed => RfidScanSessionStatus::Failed,
            ocpp_csms_server::RfidScanSessionStatus::Unspecified => RfidScanSessionStatus::Failed,
        }
    }
}
