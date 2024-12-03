use std::error::Error;
use std::fmt::Debug;
use sqlx::Pool;
use crate::charger::Charger;
use crate::data::DataStore;

#[derive(Debug)]
pub struct SqlxDataStore<DB: sqlx::Database> {
    pool: Pool<DB>
}

impl<DB: sqlx::Database> SqlxDataStore<DB> {
    pub async fn setup(pool: Pool<DB>) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {

        Ok(Self {
            pool
        })
    }
}

#[async_trait::async_trait]
impl<DB: Debug + sqlx::Database> DataStore for SqlxDataStore<DB> {
    async fn get_charger_by_id(&self, _id: &str) -> Result<Option<Charger>, Box<dyn Error + Send + Sync + 'static>> {
        todo!()
    }

    async fn get_password(&self, _id: &str) -> Result<Option<String>, Box<dyn Error + Send + Sync + 'static>> {
        Ok(None)
    }
}