use crate::Config;
use rust_ocpp::v1_6::messages::change_configuration::{
    ALLOW_OFFLINE_TX_FOR_UNKNOWN_ID, AUTHORIZATION_CACHE_ENABLED, AUTHORIZE_REMOTE_TX_REQUESTS,
    CLOCK_ALIGNED_DATA_INTERVAL, CONNECTION_TIME_OUT, LOCAL_AUTHORIZE_OFFLINE,
    LOCAL_AUTH_LIST_ENABLED, LOCAL_PRE_AUTHORIZE, MAX_ENERGY_ON_INVALID_ID,
    METER_VALUES_ALIGNED_DATA, METER_VALUES_SAMPLED_DATA, METER_VALUE_SAMPLE_INTERVAL,
    MINIMUM_STATUS_DURATION, RESET_RETRIES, STOP_TRANSACTION_ON_EV_SIDE_DISCONNECT,
    STOP_TRANSACTION_ON_INVALID_ID, TRANSACTION_MESSAGE_ATTEMPTS,
    TRANSACTION_MESSAGE_RETRY_INTERVAL, UNLOCK_CONNECTOR_ON_EV_SIDE_DISCONNECT,
    WEB_SOCKET_PING_INTERVAL,
};
use rust_ocpp::v1_6::types::Measurand;
use sqlx::FromRow;
use std::collections::BTreeMap;

#[derive(Debug, Clone, FromRow)]
pub struct ChargerSettings {
    pub authorize_transactions: bool,
    pub permanently_lock_cable_to_charger: bool,
}

impl ChargerSettings {
    pub fn get_ocpp_1_6_configuration_entries(&self, config: &Config) -> BTreeMap<String, String> {
        let mut entries = BTreeMap::new();
        entries.insert(
            ALLOW_OFFLINE_TX_FOR_UNKNOWN_ID.to_string(),
            "false".to_string(),
        );
        entries.insert(AUTHORIZATION_CACHE_ENABLED.to_string(), "false".to_string());
        entries.insert(
            AUTHORIZE_REMOTE_TX_REQUESTS.to_string(),
            "false".to_string(),
        );
        entries.insert(
            CONNECTION_TIME_OUT.to_string(),
            config
                .ocpp
                .clone()
                .unwrap_or_default()
                .message_timeout_secs
                .unwrap_or(30)
                .to_string(),
        );
        entries.insert(LOCAL_AUTHORIZE_OFFLINE.to_string(), "false".to_string());
        entries.insert(LOCAL_PRE_AUTHORIZE.to_string(), "false".to_string());
        entries.insert(MAX_ENERGY_ON_INVALID_ID.to_string(), "0".to_string());
        entries.insert(MINIMUM_STATUS_DURATION.to_string(), "0".to_string());
        entries.insert(RESET_RETRIES.to_string(), "3".to_string());
        entries.insert(
            STOP_TRANSACTION_ON_EV_SIDE_DISCONNECT.to_string(),
            "true".to_string(),
        );
        entries.insert(
            STOP_TRANSACTION_ON_INVALID_ID.to_string(),
            "false".to_string(),
        );
        entries.insert(TRANSACTION_MESSAGE_ATTEMPTS.to_string(), "3".to_string());
        entries.insert(
            TRANSACTION_MESSAGE_RETRY_INTERVAL.to_string(),
            "30".to_string(),
        );
        entries.insert(WEB_SOCKET_PING_INTERVAL.to_string(), "5".to_string());
        entries.insert(LOCAL_AUTH_LIST_ENABLED.to_string(), "true".to_string());
        entries.insert(LOCAL_AUTH_LIST_ENABLED.to_string(), "true".to_string());
        entries.insert(
            UNLOCK_CONNECTOR_ON_EV_SIDE_DISCONNECT.to_string(),
            match self.permanently_lock_cable_to_charger {
                true => "false",
                false => "true",
            }
            .to_string(),
        );

        entries.insert(CLOCK_ALIGNED_DATA_INTERVAL.to_string(), "300".to_string());
        entries.insert(
            METER_VALUES_ALIGNED_DATA.to_string(),
            [Measurand::Voltage, Measurand::Temperature].join(","),
        );
        entries.insert(METER_VALUE_SAMPLE_INTERVAL.to_string(), "30".to_string());
        entries.insert(
            METER_VALUES_SAMPLED_DATA.to_string(),
            [
                Measurand::PowerActiveExport,
                Measurand::CurrentExport,
                Measurand::CurrentOffered,
                Measurand::EnergyActiveExportRegister,
                Measurand::SoC,
            ]
            .join(","),
        );

        entries
    }
}

impl Default for ChargerSettings {
    fn default() -> Self {
        Self {
            authorize_transactions: true,
            permanently_lock_cable_to_charger: false,
        }
    }
}
