use std::error::Error;
use std::fmt::Debug;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, Pool, Postgres};
use uuid::Uuid;
use crate::charger::{ChargerData};
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
impl DataStore for SqlxDataStore<Postgres> {
    async fn get_charger_data_by_id(&self, id: &str) -> Result<Option<ChargerData>, Box<dyn Error + Send + Sync + 'static>> {
        let row = sqlx::query_as!(ChargerData, "SELECT * FROM chargers WHERE id = $1", id)
            .fetch_optional(&self.pool).await?;
        Ok(row)
    }

    async fn save_charger_data(&self, data: &ChargerData) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        sqlx::query!("
            INSERT INTO chargers (id, model, vendor, serial_number, firmware_version, iccid, imsi)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id)
            DO UPDATE SET model = $2, vendor = $3, serial_number = $4, firmware_version = $5, iccid = $6, imsi = $7
        ", data.id, data.model, data.vendor, data.serial_number, data.firmware_version, data.iccid, data.imsi)
            .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_password(&self, id: &str) -> Result<Option<String>, Box<dyn Error + Send + Sync + 'static>> {
        let row = sqlx::query!("SELECT * FROM passwords WHERE charger_id = $1", id)
            .fetch_optional(&self.pool).await?;
        Ok(row.map(|i| i.hashed_password))
    }

    async fn save_password(&self, id: &str, password: &str) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        sqlx::query!("
            INSERT INTO passwords (id, charger_id, hashed_password)
            VALUES ($1, $2, $3)
        ", Uuid::new_v4(), id, password)
            .execute(&self.pool).await?;
        Ok(())
    }

    async fn get_rfid_tag_by_hex(&self, rfid_hex: &str) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let row = sqlx::query!("SELECT * FROM rfid_tags WHERE rfid_hex = $1", rfid_hex)
            .fetch_optional(&self.pool).await?;
        Ok(row.map(|i| i.rfid_hex))
    }
}

#[derive(FromRow)]
struct PasswordRecord {
    id: Uuid,
    charger_id: String,
    hashed_password: String,
    created_at: DateTime<Utc>,
    last_used_at: Option<DateTime<Utc>>,
}