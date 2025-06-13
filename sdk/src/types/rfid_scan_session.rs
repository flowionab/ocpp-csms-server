use crate::types::rfid_scan_session_status::RfidScanSessionStatus;
use chrono::{DateTime, TimeZone, Utc};
use uuid::Uuid;

pub struct RfidScanSession {
    pub id: Uuid,
    pub charger_id: String,
    pub rfid_uid_hex: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub status: RfidScanSessionStatus,
}

impl TryFrom<crate::ocpp_csms_server::RfidScanSession> for RfidScanSession {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(session: crate::ocpp_csms_server::RfidScanSession) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(&session.id)?,
            charger_id: session.charger_id,
            rfid_uid_hex: session.rfid_uid_hex,
            status: crate::ocpp_csms_server::RfidScanSessionStatus::try_from(session.status)?
                .into(),
            expires_at: Utc
                .timestamp_millis_opt(session.expires_at as i64)
                .single()
                .ok_or("Invalid timestamp")?,
        })
    }
}
