use crate::network_interface::protocol_handle::ProtocolHandle;
use std::sync::Arc;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait ChargerFactory<T: Send + Sync> {
    async fn create_charger(
        &self,
        id: &str,
        handle: ProtocolHandle,
    ) -> Result<T, Box<dyn std::error::Error + Send + Sync>>;

    async fn on_connected(
        &self,
        charger: &Arc<tokio::sync::Mutex<T>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    async fn on_disconnected(
        &self,
        charger: &Arc<tokio::sync::Mutex<T>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
