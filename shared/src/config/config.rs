use crate::config::amqp_config::AmqpConfig;
use crate::config::authorize_config::AuthorizeConfig;
use crate::config::client_config::ClientConfig;
use crate::config::ocpp_config::OcppConfig;
use serde::{Deserialize, Serialize};

/// Main configuration structure for the application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ocpp: Option<OcppConfig>,
    pub amqp: Option<AmqpConfig>,
    pub authorize: Option<AuthorizeConfig>,
    pub client: Option<ClientConfig>,
}

impl Config {
    pub fn print_config_warnings(&self) {
        if let Some(ocpp) = &self.ocpp {
            ocpp.print_config_warnings()
        }
    }

    pub fn client_url(&self) -> Option<&str> {
        self.client.as_ref().and_then(|c| c.url.as_deref())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ocpp: Some(OcppConfig::default()),
            amqp: None,
            authorize: None,
            client: None,
        }
    }
}
