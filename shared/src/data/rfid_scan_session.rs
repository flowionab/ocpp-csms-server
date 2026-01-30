use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct RfidScanSession {
    pub id: Uuid,
    pub charger_id: String,
    pub rfid_uid_hex: Option<String>,
    pub created_at: DateTime<Utc>,
    pub tag_scanned_at: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
    pub ocpp_reservation_id: i32,
}
