use crate::charger_data::ChargerData;
use std::fmt::Debug;

#[async_trait::async_trait]
pub trait DataStore: Send + Sync + Debug {
    async fn get_charger_data_by_id(
        &self,
        id: &str,
    ) -> Result<Option<ChargerData>, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn save_charger_data(
        &self,
        data: &ChargerData,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn get_password(
        &self,
        id: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn save_password(
        &self,
        id: &str,
        password: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn get_rfid_tag_by_hex(
        &self,
        rfid_hex: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync + 'static>>;
}
