use crate::event::sampled_value::SampledValue;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeterValue {
    pub timestamp: DateTime<Utc>,
    pub sampled_value: Vec<SampledValue>,
}
