use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizeConfig {
    pub default_authorize_transactions: Option<bool>,
}

impl Default for AuthorizeConfig {
    fn default() -> Self {
        Self {
            default_authorize_transactions: Some(true),
        }
    }
}
