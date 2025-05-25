use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcppConfig {
    /// This will disable any charger validation. Should only be used for testing
    pub disable_charger_auth: Option<bool>,
    pub message_timeout_secs: Option<u64>,
}

impl OcppConfig {
    pub fn print_config_warnings(&self) {
        if let Some(disable_charger_auth) = self.disable_charger_auth {
            if disable_charger_auth {
                warn!(
                    "'ocpp.disable_charger_auth' is enabled, no charger authentication will happen"
                )
            }
        }
    }
}

impl Default for OcppConfig {
    fn default() -> Self {
        Self {
            disable_charger_auth: Some(false),
            message_timeout_secs: Some(30),
        }
    }
}
