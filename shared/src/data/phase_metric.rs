use crate::data::metric::Metric;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhaseMetric<T> {
    pub l1: Metric<T>,
    pub l2: Metric<T>,
    pub l3: Metric<T>,
}
