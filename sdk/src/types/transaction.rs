use chrono::{DateTime, TimeZone, Utc};
use uuid::Uuid;

pub struct Transaction {
    pub id: Uuid,
    pub charger_id: String,
    pub ocpp_transaction_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub watt_charged: i32,
    pub is_authorized: bool,
}

impl TryFrom<crate::ocpp_csms_server::Transaction> for Transaction {
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

    fn try_from(value: crate::ocpp_csms_server::Transaction) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::parse_str(&value.id)?,
            charger_id: value.charger_id,
            ocpp_transaction_id: value.ocpp_transaction_id,
            start_time: Utc
                .timestamp_millis_opt(value.start_time)
                .latest()
                .ok_or_else(|| format!("Invalid start time: {}", value.start_time))?,
            end_time: value
                .end_time
                .map(|end_time| {
                    Utc.timestamp_millis_opt(end_time)
                        .latest()
                        .ok_or_else(|| format!("Invalid end time: {}", end_time))
                })
                .transpose()?,
            watt_charged: value.watt_charged,
            is_authorized: value.is_authorized,
        })
    }
}
