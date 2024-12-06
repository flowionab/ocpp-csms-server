use rust_ocpp::v1_6::messages::get_configuration::GetConfigurationResponse;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Ocpp1_6ConfigurationValue {
    pub value: Option<String>,
    pub read_only: bool,
}

#[derive(Debug, Clone)]
pub struct Ocpp1_6Configuration {
    configurations: BTreeMap<String, Ocpp1_6ConfigurationValue>,
}

impl Ocpp1_6Configuration {
    pub const ALLOW_OFFLINE_TX_FOR_UNKNOWN_ID: &'static str = "AllowOfflineTxForUnknownId";
    pub const AUTHORIZATION_CACHE_ENABLED: &'static str = "AuthorizationCacheEnabled";
    pub const AUTHORIZE_REMOTE_TX_REQUESTS: &'static str = "AuthorizeRemoteTxRequests";
    pub const BLINK_REPEAT: &'static str = "BlinkRepeat";
    pub const CLOCK_ALIGNED_DATA_INTERVAL: &'static str = "ClockAlignedDataInterval";
    pub const CONNECTION_TIME_OUT: &'static str = "ConnectionTimeOut";
    pub const CONNECTOR_PHASE_ROTATION: &'static str = "ConnectorPhaseRotation";
    pub const CONNECTOR_PHASE_ROTATION_MAX_LENGTH: &'static str = "ConnectorPhaseRotationMaxLength";
    pub const GET_CONFIGURATION_MAX_KEYS: &'static str = "GetConfigurationMaxKeys";
    pub const HEARTBEAT_INTERVAL: &'static str = "HeartbeatInterval";
    pub const LIGHT_INTENSITY: &'static str = "LightIntensity";
    pub const LOCAL_AUTHORIZE_OFFLINE: &'static str = "LocalAuthorizeOffline";
    pub const LOCAL_PRE_AUTHORIZE: &'static str = "LocalPreAuthorize";
    pub const MAX_ENERGY_ON_INVALID_ID: &'static str = "MaxEnergyOnInvalidId";
    pub const METER_VALUES_ALIGNED_DATA: &'static str = "MeterValuesAlignedData";
    pub const METER_VALUES_ALIGNED_DATA_MAX_LENGTH: &'static str =
        "MeterValuesAlignedDataMaxLength";
    pub const METER_VALUES_SAMPLED_DATA: &'static str = "MeterValuesSampledData";
    pub const METER_VALUES_SAMPLED_DATA_MAX_LENGTH: &'static str =
        "MeterValuesSampledDataMaxLength";
    pub const METER_VALUE_SAMPLE_INTERVAL: &'static str = "MeterValueSampleInterval";
    pub const MINIMUM_STATUS_DURATION: &'static str = "MinimumStatusDuration";
    pub const NUMBER_OF_CONNECTORS: &'static str = "NumberOfConnectors";
    pub const RESET_RETRIES: &'static str = "ResetRetries";
    pub const STOP_TRANSACTION_ON_EV_SIDE_DISCONNECT: &'static str =
        "StopTransactionOnEVSideDisconnect";
    pub const STOP_TRANSACTION_ON_INVALID_ID: &'static str = "StopTransactionOnInvalidId";
    pub const STOP_TXN_ALIGNED_DATA: &'static str = "StopTxnAlignedData";
    pub const STOP_TXN_ALIGNED_DATA_MAX_LENGTH: &'static str = "StopTxnAlignedDataMaxLength";
    pub const STOP_TXN_SAMPLED_DATA: &'static str = "StopTxnSampledData";
    pub const STOP_TXN_SAMPLED_DATA_MAX_LENGTH: &'static str = "StopTxnSampledDataMaxLength";
    pub const SUPPORTED_FEATURE_PROFILES: &'static str = "SupportedFeatureProfiles";
    pub const SUPPORTED_FEATURE_PROFILES_MAX_LENGTH: &'static str =
        "SupportedFeatureProfilesMaxLength";
    pub const TRANSACTION_MESSAGE_ATTEMPTS: &'static str = "TransactionMessageAttempts";
    pub const TRANSACTION_MESSAGE_RETRY_INTERVAL: &'static str = "TransactionMessageRetryInterval";
    pub const UNLOCK_CONNECTOR_ON_EV_SIDE_DISCONNECT: &'static str =
        "UnlockConnectorOnEVSideDisconnect";
    pub const WEB_SOCKET_PING_INTERVAL: &'static str = "WebSocketPingInterval";
    pub const LOCAL_AUTH_LIST_ENABLED: &'static str = "LocalAuthListEnabled";
    pub const LOCAL_AUTH_LIST_MAX_LENGTH: &'static str = "LocalAuthListMaxLength";
    pub const SEND_LOCAL_LIST_MAX_LENGTH: &'static str = "SendLocalListMaxLength";
    pub const RESERVE_CONNECTOR_ZERO_SUPPORTED: &'static str = "ReserveConnectorZeroSupported";
    pub const CHARGE_PROFILE_MAX_STACK_LEVEL: &'static str = "ChargeProfileMaxStackLevel";
    pub const CHARGING_SCHEDULE_ALLOWED_CHARGING_RATE_UNIT: &'static str =
        "ChargingScheduleAllowedChargingRateUnit";
    pub const CHARGING_SCHEDULE_MAX_PERIODS: &'static str = "ChargingScheduleMaxPeriods";
    pub const CONNECTOR_SWITCH3TO1PHASE_SUPPORTED: &'static str =
        "ConnectorSwitch3to1PhaseSupported";
    pub const MAX_CHARGING_PROFILES_INSTALLED: &'static str = "MaxChargingProfilesInstalled";

    pub fn new() -> Self {
        Self {
            configurations: BTreeMap::new(),
        }
    }

    pub fn from_full_get_configuration_response(response: &GetConfigurationResponse) -> Self {
        let mut configurations = BTreeMap::new();

        if let Some(list) = &response.configuration_key {
            for item in list {
                configurations.insert(
                    item.key.to_string(),
                    Ocpp1_6ConfigurationValue {
                        value: item.value.clone(),
                        read_only: item.readonly,
                    },
                );
            }
        }

        Self { configurations }
    }

    pub fn get_configuration(&self, name: &str) -> Option<&Ocpp1_6ConfigurationValue> {
        self.configurations.get(name)
    }
}
