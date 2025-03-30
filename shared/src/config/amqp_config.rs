use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AmqpConfig {
    /// This will disable any charger validation. Should only be used for testing
    pub enabled: bool,
}
