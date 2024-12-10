use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display)]
pub enum Status {
    Available,
    Occupied,
    Reserved,
    Unavailable,
    Faulted,
}

impl Default for Status {
    fn default() -> Self {
        Self::Available
    }
}
