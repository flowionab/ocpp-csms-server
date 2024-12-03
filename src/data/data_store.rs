use std::fmt::Debug;
use crate::charger::Charger;

#[async_trait::async_trait]
pub trait DataStore: Send + Sync + Debug {
    async fn get_charger_by_id(&self, id: &str) -> Result<Option<Charger>, Box<dyn std::error::Error + Send + Sync + 'static>>;
    async fn get_password(&self, id: &str) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync + 'static>>;
}