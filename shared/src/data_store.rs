use crate::charger_data::ChargerData;
use crate::rfid_scan_session::RfidScanSession;
use crate::transaction::Transaction;
use crate::ChargerConnectionInfo;
use chrono::{DateTime, Utc};
use std::error::Error;
use std::fmt::Debug;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait DataStore: Send + Sync + Debug {
    async fn get_charger_data_by_id(
        &self,
        id: &str,
    ) -> Result<Option<ChargerData>, Box<dyn Error + Send + Sync + 'static>>;

    async fn get_chargers(
        &self,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<ChargerData>, Box<dyn Error + Send + Sync + 'static>>;

    async fn count_chargers(&self) -> Result<i64, Box<dyn Error + Send + Sync + 'static>>;

    async fn save_charger_data(
        &self,
        data: &ChargerData,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    async fn get_password(
        &self,
        id: &str,
    ) -> Result<Option<String>, Box<dyn Error + Send + Sync + 'static>>;

    async fn save_password(
        &self,
        id: &str,
        password: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    async fn get_rfid_tag_by_hex(
        &self,
        rfid_hex: &str,
    ) -> Result<Option<String>, Box<dyn Error + Send + Sync + 'static>>;

    async fn get_charger_connection_info(
        &self,
        id: &str,
    ) -> Result<Option<ChargerConnectionInfo>, Box<dyn Error + Send + Sync + 'static>>;

    async fn update_charger_connection_info(
        &self,
        id: &str,
        is_online: bool,
        node_address: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    async fn create_charger(
        &self,
        charger_id: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    async fn create_transaction(
        &self,
        charger_id: &str,
        evse_id: Uuid,
        ocpp_transaction_id: &str,
        start_time: chrono::DateTime<chrono::Utc>,
        is_authorized: bool,
    ) -> Result<Transaction, Box<dyn Error + Send + Sync + 'static>>;

    async fn get_ongoing_transaction(
        &self,
        charger_id: &str,
        evse_id: Uuid,
    ) -> Result<Option<Transaction>, Box<dyn Error + Send + Sync + 'static>>;

    async fn end_transaction(
        &self,
        charger_id: &str,
        ocpp_transaction_id: &str,
        end_time: chrono::DateTime<chrono::Utc>,
    ) -> Result<Option<Transaction>, Box<dyn Error + Send + Sync + 'static>>;

    async fn update_transaction_watt_charged(
        &self,
        charger_id: &str,
        ocpp_transaction_id: &str,
        watt_charged: i32,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    async fn update_transaction_is_authorized(
        &self,
        transaction_id: Uuid,
        is_authorized: bool,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    async fn update_transaction_meter_start(
        &self,
        transaction_id: Uuid,
        meter_start: i32,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    async fn get_transaction(
        &self,
        transaction_id: Uuid,
    ) -> Result<Option<Transaction>, Box<dyn Error + Send + Sync + 'static>>;

    async fn get_transaction_by_ocpp_id(
        &self,
        transaction_ocpp_id: &str,
    ) -> Result<Option<Transaction>, Box<dyn Error + Send + Sync + 'static>>;

    async fn create_rfid_scan_session(
        &self,
        charger_id: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<RfidScanSession, Box<dyn Error + Send + Sync + 'static>>;

    async fn get_ongoing_rfid_scanning_session(
        &self,
        charger_id: &str,
    ) -> Result<Option<RfidScanSession>, Box<dyn Error + Send + Sync + 'static>>;

    async fn save_scanned_tag_to_rfid_scan_session(
        &self,
        session_id: Uuid,
        rfid_uid_hex: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;

    async fn get_rfid_scanning_session(
        &self,
        session_id: Uuid,
    ) -> Result<Option<RfidScanSession>, Box<dyn Error + Send + Sync + 'static>>;
}
