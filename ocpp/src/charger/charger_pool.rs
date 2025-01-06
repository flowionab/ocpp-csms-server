use crate::charger::Charger;
use std::collections::BTreeMap;
use std::sync::{Arc, Weak};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ChargerPool {
    chargers: Arc<Mutex<BTreeMap<String, Weak<Mutex<Charger>>>>>,
}

impl ChargerPool {
    pub fn new() -> Self {
        Self {
            chargers: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    pub async fn insert(&self, id: &str, charger: &Arc<Mutex<Charger>>) {
        let mut lock = self.chargers.lock().await;
        lock.insert(id.to_string(), Arc::downgrade(charger));
    }

    pub async fn get(&self, charger_id: &str) -> Option<Arc<Mutex<Charger>>> {
        let lock = self.chargers.lock().await;
        lock.get(charger_id).and_then(|i| i.upgrade())
    }
}
