use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, Default, FromRow)]
pub struct ChargerConnectionInfo {
    pub id: String,
    pub last_seen: DateTime<Utc>,
    pub is_online: bool,
    pub node_address: String,
}
