use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmqpConfig {
    /// This will disable any charger validation. Should only be used for testing
    pub enabled: bool,
}

impl Default for AmqpConfig {
    fn default() -> Self {
        Self { enabled: false }
    }
}
