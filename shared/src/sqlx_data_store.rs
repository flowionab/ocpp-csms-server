use crate::charger_data::ChargerData;
use crate::data_store::DataStore;
use crate::transaction::Transaction;
use crate::ChargerConnectionInfo;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, Pool, Postgres};
use std::error::Error;
use std::fmt::Debug;
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug)]
pub struct SqlxDataStore<DB: sqlx::Database> {
    pool: Pool<DB>,
}

impl<DB: sqlx::Database> SqlxDataStore<DB> {
    pub async fn setup(pool: Pool<DB>) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        Ok(Self { pool })
    }
}

#[async_trait::async_trait]
impl DataStore for SqlxDataStore<Postgres> {
    #[instrument]
    async fn get_charger_data_by_id(
        &self,
        id: &str,
    ) -> Result<Option<ChargerData>, Box<dyn Error + Send + Sync + 'static>> {
        let row = sqlx::query!("SELECT * FROM chargers WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row
            .map(|record| {
                Ok::<_, Box<dyn Error + Send + Sync>>(ChargerData {
                    id: record.id,
                    model: record.model,
                    vendor: record.vendor,
                    serial_number: record.serial_number,
                    firmware_version: record.firmware_version,
                    iccid: record.iccid,
                    imsi: record.imsi,
                    ocpp1_6configuration: record
                        .ocpp1_6configuration
                        .map(|j| serde_json::from_str(&j).unwrap_or_default()),
                    evses: record
                        .evses
                        .map(|j| serde_json::from_str(&j).unwrap_or_default())
                        .unwrap_or_default(),
                    settings: serde_json::from_str(&record.settings)?,
                })
            })
            .transpose()?)
    }

    async fn get_chargers(
        &self,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<ChargerData>, Box<dyn Error + Send + Sync + 'static>> {
        let rows = sqlx::query!(
            "SELECT * FROM chargers ORDER BY serial_number DESC LIMIT $1 OFFSET $2",
            page_size,
            page * page_size
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|record| {
                Ok::<_, Box<dyn Error + Send + Sync>>(ChargerData {
                    id: record.id,
                    model: record.model,
                    vendor: record.vendor,
                    serial_number: record.serial_number,
                    firmware_version: record.firmware_version,
                    iccid: record.iccid,
                    imsi: record.imsi,
                    ocpp1_6configuration: record
                        .ocpp1_6configuration
                        .map(|j| serde_json::from_str(&j).unwrap_or_default()),
                    evses: record
                        .evses
                        .map(|j| serde_json::from_str(&j).unwrap_or_default())
                        .unwrap_or_default(),
                    settings: serde_json::from_str(&record.settings)?,
                })
            })
            .collect::<Result<Vec<_>, _>>()?)
    }

    async fn count_chargers(&self) -> Result<i64, Box<dyn Error + Send + Sync + 'static>> {
        let count: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM chargers")
            .fetch_one(&self.pool)
            .await?
            .ok_or("No count returned")?;
        Ok(count)
    }

    #[instrument]
    async fn save_charger_data(
        &self,
        data: &ChargerData,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let ocpp1_6configuration = serde_json::to_string(&data.ocpp1_6configuration)?;
        let outlets = serde_json::to_string(&data.evses)?;
        sqlx::query!("
            INSERT INTO chargers (id, model, vendor, serial_number, firmware_version, iccid, imsi, ocpp1_6configuration, evses)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id)
            DO UPDATE SET model = $2, vendor = $3, serial_number = $4, firmware_version = $5, iccid = $6, imsi = $7, ocpp1_6configuration = $8, evses = $9
        ", data.id, data.model, data.vendor, data.serial_number, data.firmware_version, data.iccid, data.imsi, ocpp1_6configuration, outlets)
            .execute(&self.pool).await?;
        Ok(())
    }

    #[instrument]
    async fn get_password(
        &self,
        id: &str,
    ) -> Result<Option<String>, Box<dyn Error + Send + Sync + 'static>> {
        let row = sqlx::query!("SELECT * FROM passwords WHERE charger_id = $1", id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|i| i.hashed_password))
    }

    #[instrument]
    async fn save_password(
        &self,
        id: &str,
        password: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        sqlx::query!(
            "
            INSERT INTO passwords (id, charger_id, hashed_password)
            VALUES ($1, $2, $3)
        ",
            Uuid::new_v4(),
            id,
            password
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    #[instrument]
    async fn get_rfid_tag_by_hex(
        &self,
        rfid_hex: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let row = sqlx::query!("SELECT * FROM rfid_tags WHERE rfid_hex = $1", rfid_hex)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|i| i.rfid_hex))
    }

    async fn get_charger_connection_info(
        &self,
        id: &str,
    ) -> Result<Option<ChargerConnectionInfo>, Box<dyn Error + Send + Sync + 'static>> {
        let row = sqlx::query_as!(
            ChargerConnectionInfo,
            "SELECT * FROM charger_connection_info WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    async fn update_charger_connection_info(
        &self,
        id: &str,
        is_online: bool,
        node_address: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        sqlx::query!(
            "
            INSERT INTO charger_connection_info (id, node_address, is_online, last_seen)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id)
            DO UPDATE SET node_address = $2, is_online = $3, last_seen = $4
        ",
            id,
            node_address,
            is_online,
            Utc::now()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn create_charger(
        &self,
        charger_id: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        sqlx::query!(
            "
            INSERT INTO chargers (id)
            VALUES ($1)
        ",
            charger_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn create_transaction(
        &self,
        charger_id: &str,
        evse_id: Uuid,
        ocpp_transaction_id: &str,
        start_time: DateTime<Utc>,
        is_authorized: bool,
    ) -> Result<Transaction, Box<dyn Error + Send + Sync + 'static>> {
        Ok(sqlx::query!(
            "
            INSERT INTO transactions (id, charger_id, evse_id, start_time, is_authorized, ocpp_transaction_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
        ",
            Uuid::new_v4(),
            charger_id,
            evse_id,
            start_time,
            is_authorized,
            ocpp_transaction_id
        )
        .fetch_one(&self.pool)
        .await
        .map(|record| Transaction {
            id: record.id,
            charger_id: charger_id.to_string(),
            evse_id,
            ocpp_transaction_id: ocpp_transaction_id.to_string(),
            start_time,
            end_time: None,
            watt_charged: 0,
            energy_meter_at_start: None,
            is_authorized,
        })?)
    }

    async fn get_ongoing_transaction(
        &self,
        charger_id: &str,
        evse_id: Uuid,
    ) -> Result<Option<Transaction>, Box<dyn Error + Send + Sync + 'static>> {
        Ok(sqlx::query_as!(
            Transaction,
            "
                SELECT * FROM transactions WHERE charger_id = $1 AND evse_id = $2 AND end_time IS NULL LIMIT 1
            ",
            charger_id,
            evse_id
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    async fn end_transaction(
        &self,
        charger_id: &str,
        ocpp_transaction_id: &str,
        end_time: DateTime<Utc>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        sqlx::query!(
            "
                UPDATE transactions SET end_time = $1 WHERE charger_id = $2 AND ocpp_transaction_id = $3
            ",
            end_time,
            charger_id,
            ocpp_transaction_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update_transaction_watt_charged(
        &self,
        charger_id: &str,
        ocpp_transaction_id: &str,
        watt_charged: i32,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        sqlx::query!(
            "
                UPDATE transactions SET watt_charged = $1 WHERE charger_id = $2 AND ocpp_transaction_id = $3
            ",
            watt_charged,
            charger_id,
            ocpp_transaction_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update_transaction_is_authorized(
        &self,
        transaction_id: Uuid,
        is_authorized: bool,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        sqlx::query!(
            "
                UPDATE transactions SET is_authorized = $1 WHERE id = $2
            ",
            is_authorized,
            transaction_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update_transaction_meter_start(
        &self,
        transaction_id: Uuid,
        meter_start: i32,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        sqlx::query!(
            "
                UPDATE transactions SET energy_meter_at_start = $1 WHERE id = $2
            ",
            meter_start,
            transaction_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_transaction(
        &self,
        transaction_id: Uuid,
    ) -> Result<Option<Transaction>, Box<dyn Error + Send + Sync + 'static>> {
        sqlx::query_as!(
            Transaction,
            "
                SELECT * FROM transactions WHERE id = $1
            ",
            transaction_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.into())
    }
}

#[derive(FromRow, Debug, Clone)]
struct PasswordRecord {
    #[allow(dead_code)]
    id: Uuid,

    #[allow(dead_code)]
    charger_id: String,

    #[allow(dead_code)]
    hashed_password: String,

    #[allow(dead_code)]
    created_at: DateTime<Utc>,

    #[allow(dead_code)]
    last_used_at: Option<DateTime<Utc>>,
}
