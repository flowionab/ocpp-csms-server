use crate::charger::{Charger, ChargerPool};
use crate::event::EventManager;
use crate::network_interface::ProtocolHandle;
use crate::ocpp_csms_server_client::csms_server_client_client::CsmsServerClientClient;
use shared::Config;
use shared::data_store::DataStore;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Channel;

pub struct ChargerFactory {
    config: Arc<Config>,
    data_store: Arc<dyn DataStore + Send + Sync>,
    node_address: String,
    easee_master_password: Option<String>,
    event_manager: EventManager,
    charger_pool: ChargerPool,
    csms_server_client: Option<CsmsServerClientClient<Channel>>,
}

impl ChargerFactory {
    pub fn new(
        config: Arc<Config>,
        data_store: Arc<dyn DataStore + Send + Sync>,
        node_address: &str,
        easee_master_password: Option<String>,
        event_manager: &EventManager,
        charger_pool: &ChargerPool,
        csms_server_client: &Option<CsmsServerClientClient<Channel>>,
    ) -> Self {
        Self {
            config,
            data_store,
            node_address: node_address.to_string(),
            easee_master_password,
            event_manager: event_manager.clone(),
            charger_pool: charger_pool.clone(),
            csms_server_client: csms_server_client.clone(),
        }
    }
}

#[async_trait::async_trait]
impl crate::network_interface::ChargerFactory<Charger> for ChargerFactory {
    async fn create_charger(
        &self,
        id: &str,
        handle: ProtocolHandle,
    ) -> Result<Charger, Box<dyn Error + Send + Sync>> {
        Charger::setup(
            id,
            Arc::clone(&self.config),
            handle,
            Arc::clone(&self.data_store),
            &self.node_address,
            self.easee_master_password.clone(),
            self.event_manager.clone(),
            self.csms_server_client.clone(),
        )
        .await
    }

    async fn on_connected(
        &self,
        charger: &Arc<Mutex<Charger>>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.charger_pool
            .insert(&charger.lock().await.id, charger)
            .await;
        Ok(())
    }

    async fn on_disconnected(
        &self,
        charger: &Arc<Mutex<Charger>>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.charger_pool.remove(&charger.lock().await.id).await;
        Ok(())
    }
}
