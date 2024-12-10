use crate::config::ocpp_config::OcppConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ocpp: Option<OcppConfig>,
}

impl Config {
    pub fn print_config_warnings(&self) {
        if let Some(ocpp) = &self.ocpp {
            ocpp.print_config_warnings()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ocpp: Some(OcppConfig::default()),
        }
    }
}
