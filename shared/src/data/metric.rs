use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Metric<T> {
    pub value: T,
    pub measured_at: Option<DateTime<Utc>>,
}
