use crate::charger::Charger;
use lazy_static::lazy_static;
use prometheus::{register_gauge, Gauge};
use std::collections::BTreeMap;
use std::sync::{Arc, Weak};
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::interval;
use tracing::info;

lazy_static! {
    static ref CONNECTED_CHARGERS: Gauge = register_gauge!(
        "ocpp_csms_server_connected_chargers",
        "Number of connected chargers"
    )
    .unwrap();
}

type ChargerMap = BTreeMap<String, Weak<Mutex<Charger>>>;

#[derive(Clone)]
pub struct ChargerPool {
    chargers: Arc<Mutex<ChargerMap>>,
}

impl ChargerPool {
    pub fn new() -> Self {
        let chargers = Arc::new(Mutex::new(BTreeMap::new()));

        tokio::spawn(Self::gc_chargers(chargers.clone()));

        Self { chargers }
    }

    async fn gc_chargers(chargers: Arc<Mutex<BTreeMap<String, Weak<Mutex<Charger>>>>>) {
        let mut interval = interval(Duration::from_secs(300));

        loop {
            interval.tick().await;
            Self::remove_gone_chargers(&chargers).await;
        }
    }

    async fn remove_gone_chargers(chargers: &Arc<Mutex<ChargerMap>>) {
        let mut lock = chargers.lock().await;
        lock.retain(|_, weak| weak.strong_count() > 0);
        CONNECTED_CHARGERS.set(lock.len() as f64);
        info!(
            connected_chargers = lock.len(),
            "currently connected chargers"
        );
    }

    pub async fn insert(&self, id: &str, charger: &Arc<Mutex<Charger>>) {
        let mut lock = self.chargers.lock().await;
        lock.insert(id.to_string(), Arc::downgrade(charger));
    }

    pub async fn get(&self, charger_id: &str) -> Option<Arc<Mutex<Charger>>> {
        let lock = self.chargers.lock().await;
        lock.get(charger_id).and_then(|i| i.upgrade())
    }

    pub async fn remove(&self, charger_id: &str) {
        let mut lock = self.chargers.lock().await;
        lock.remove(charger_id);
        CONNECTED_CHARGERS.set(lock.len() as f64);
    }
}
