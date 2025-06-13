use crate::charger::charger_model::ChargerModel;
use crate::charger::ocpp1_6::{create_transaction_id, update_charger_from_meter_values_request};
use crate::charger::Charger;
use crate::network_interface::Ocpp16RequestReceiver;
use crate::ocpp_csms_server_client;
use crate::ocpp_csms_server_client::authorize_request::Authorization;
use crate::ocpp_csms_server_client::authorize_response;
use bcrypt::DEFAULT_COST;
use chrono::{DateTime, TimeZone, Utc};
use ocpp_client::ocpp_1_6::OCPP1_6Error;
use ocpp_csms_server_sdk::event;
use ocpp_csms_server_sdk::event::{
    EventPayload, EvseInfo, TransactionEvent, TransactionEventTriggerReason, TransactionEventType,
    TransactionInfo, TransactionStartedEvent, TransactionStoppedEvent,
};
use rand::distr::Alphanumeric;
use rand::Rng;
use rust_ocpp::v1_6::messages::authorize::{AuthorizeRequest, AuthorizeResponse};
use rust_ocpp::v1_6::messages::boot_notification::{
    BootNotificationRequest, BootNotificationResponse,
};
use rust_ocpp::v1_6::messages::change_configuration::ChangeConfigurationRequest;
use rust_ocpp::v1_6::messages::data_transfer::{DataTransferRequest, DataTransferResponse};
use rust_ocpp::v1_6::messages::diagnostics_status_notification::{
    DiagnosticsStatusNotificationRequest, DiagnosticsStatusNotificationResponse,
};
use rust_ocpp::v1_6::messages::firmware_status_notification::{
    FirmwareStatusNotificationRequest, FirmwareStatusNotificationResponse,
};
use rust_ocpp::v1_6::messages::get_configuration::GetConfigurationRequest;
use rust_ocpp::v1_6::messages::heart_beat::{HeartbeatRequest, HeartbeatResponse};
use rust_ocpp::v1_6::messages::meter_values::{MeterValuesRequest, MeterValuesResponse};
use rust_ocpp::v1_6::messages::reset::ResetRequest;
use rust_ocpp::v1_6::messages::start_transaction::{
    StartTransactionRequest, StartTransactionResponse,
};
use rust_ocpp::v1_6::messages::status_notification::{
    StatusNotificationRequest, StatusNotificationResponse,
};
use rust_ocpp::v1_6::messages::stop_transaction::{
    StopTransactionRequest, StopTransactionResponse,
};
use rust_ocpp::v1_6::messages::trigger_message::TriggerMessageRequest;
use rust_ocpp::v1_6::types::{
    AuthorizationStatus, ChargePointStatus, ConfigurationStatus, DataTransferStatus, IdTagInfo,
    Measurand, MessageTrigger, RegistrationStatus, ResetRequestStatus, ResetResponseStatus,
    TriggerMessageStatus,
};
use shared::{ConnectorData, ConnectorStatus, ConnectorType, EvseData, Ocpp1_6Configuration};
use std::error::Error;
use tracing::{error, info, warn};
use uuid::Uuid;

const CENTRAL_TAG: &str = "central";

impl Charger {
    pub async fn validate_rfid_tag_ocpp_1_6(&self, tag: &str) -> Result<IdTagInfo, OCPP1_6Error> {
        // Always accepted tag for central management
        if tag == CENTRAL_TAG {
            info!(
                charger_id = self.id,
                rfid_uid_hex = tag,
                "Central management tag accepted"
            );
            Ok(IdTagInfo {
                expiry_date: None,
                parent_id_tag: None,
                status: AuthorizationStatus::Accepted,
            })
        } else if self.data.settings.authorize_transactions {
            match self.csms_server_client.clone() {
                None => {
                    warn!(
                        charger_id = self.id,
                        "CSMS server client is not set, cannot validate RFID tag"
                    );
                    return Err(OCPP1_6Error::new_internal_str(
                        "CSMS server client is not set",
                    ));
                }
                Some(mut client) => {
                    let response = client
                        .authorize(ocpp_csms_server_client::AuthorizeRequest {
                            additional_info: None,
                            authorization: Some(Authorization::RfidHex(tag.to_string())),
                        })
                        .await
                        .map_err(|e| {
                            warn!(
                                charger_id = self.id,
                                error_message = e.to_string(),
                                rfid_uid_hex = tag,
                                "failed to authorize RFID tag"
                            );
                            OCPP1_6Error::new_internal(&e)
                        })?;

                    let payload = response.into_inner();

                    let expiry_date =
                        payload
                            .cache_expiration_timestamp_seconds
                            .and_then(|timestamp| {
                                // Utc from timestamp seconds
                                Utc.timestamp_opt(timestamp as i64, 0).single()
                            });

                    let status =
                        ocpp_csms_server_client::authorize_response::AuthorizationStatus::try_from(
                            payload.status,
                        )
                        .unwrap_or_default()
                        .into();

                    info!(
                        charger_id = self.id,
                        rfid_uid_hex = tag,
                        status = ?status,
                        expiry_date = ?expiry_date,
                        "RFID tag authorization response received"
                    );

                    Ok(IdTagInfo {
                        expiry_date,
                        parent_id_tag: None,
                        status,
                    })
                }
            }
        } else {
            // If authorization is disabled, then we just accept all tags
            info!(
                charger_id = self.id,
                rfid_uid_hex = tag,
                "Authorization is disabled, accepting RFID tag"
            );
            Ok(IdTagInfo {
                expiry_date: None,
                parent_id_tag: None,
                status: AuthorizationStatus::Accepted,
            })
        }
    }

    pub async fn send_meter_values_event_ocpp_1_6(
        &mut self,
        request: &MeterValuesRequest,
    ) -> Result<(), OCPP1_6Error> {
        if let Some(transaction_id) = request.transaction_id {
            if let Some(transaction) = self
                .transaction_by_ocpp_id(transaction_id)
                .await
                .map_err(|e| OCPP1_6Error::new_internal(&e))?
            {
                let payload = EventPayload::TransactionEvent(TransactionEvent {
                    charger_id: self.id.to_string(),
                    timestamp: Utc::now(),
                    event_type: TransactionEventType::Updated,
                    trigger_reason: TransactionEventTriggerReason::MeterValuePeriodic,
                    number_of_phases_used: None,
                    cable_max_current: None,
                    reservation_id: None,
                    transaction_info: TransactionInfo {
                        id: transaction.id,
                        charging_state: None,
                        time_spent_charging: None,
                        stopped_reason: None,
                    },
                    evse: EvseInfo {
                        id: transaction.evse_id,
                        connector_id: Default::default(),
                    },
                    meter_values: request
                        .meter_value
                        .iter()
                        .map(|mv| event::MeterValue {
                            timestamp: mv.timestamp,
                            sampled_value: mv
                                .sampled_value
                                .iter()
                                .filter_map(|sv| match sv.measurand.clone().unwrap_or_default() {
                                    Measurand::CurrentExport => {
                                        Some(event::SampledValue::CurrentExport {
                                            ampere: sv.value.parse().unwrap_or_default(),
                                            context: (),
                                            phase: (),
                                            location: (),
                                            signed_meter_value: (),
                                        })
                                    }
                                    Measurand::CurrentImport => {
                                        Some(event::SampledValue::CurrentImport)
                                    }
                                    Measurand::CurrentOffered => {
                                        Some(event::SampledValue::CurrentOffered)
                                    }
                                    Measurand::EnergyActiveExportRegister => {
                                        Some(event::SampledValue::EnergyActiveExportRegister)
                                    }
                                    Measurand::EnergyActiveImportRegister => {
                                        Some(event::SampledValue::EnergyActiveImportRegister)
                                    }
                                    Measurand::EnergyReactiveExportRegister => {
                                        Some(event::SampledValue::EnergyReactiveExportRegister)
                                    }
                                    Measurand::EnergyReactiveImportRegister => {
                                        Some(event::SampledValue::EnergyReactiveImportRegister)
                                    }
                                    Measurand::EnergyActiveExportInterval => {
                                        Some(event::SampledValue::EnergyActiveExportInterval)
                                    }
                                    Measurand::EnergyActiveImportInterval => {
                                        Some(event::SampledValue::EnergyActiveImportInterval)
                                    }
                                    Measurand::EnergyReactiveExportInterval => {
                                        Some(event::SampledValue::EnergyReactiveExportInterval)
                                    }
                                    Measurand::EnergyReactiveImportInterval => {
                                        Some(event::SampledValue::EnergyReactiveImportInterval)
                                    }
                                    Measurand::Frequency => Some(event::SampledValue::Frequency),
                                    Measurand::PowerActiveExport => {
                                        Some(event::SampledValue::PowerActiveExport)
                                    }
                                    Measurand::PowerActiveImport => {
                                        Some(event::SampledValue::PowerActiveImport)
                                    }
                                    Measurand::PowerFactor => {
                                        Some(event::SampledValue::PowerFactor)
                                    }
                                    Measurand::PowerOffered => {
                                        Some(event::SampledValue::PowerOffered)
                                    }
                                    Measurand::PowerReactiveExport => {
                                        Some(event::SampledValue::PowerReactiveExport)
                                    }
                                    Measurand::PowerReactiveImport => {
                                        Some(event::SampledValue::PowerReactiveImport)
                                    }
                                    Measurand::Rpm => None,
                                    Measurand::SoC => Some(event::SampledValue::SoC),
                                    Measurand::Temperature => None,
                                    Measurand::Voltage => Some(event::SampledValue::Voltage),
                                })
                                .collect(),
                        })
                        .collect(),
                });

                self.event_manager.send_event(payload).await;
            } else {
                return Err(OCPP1_6Error::new_internal_str("Transaction not found"));
            }
        }

        Ok(())
    }

    pub async fn start_transaction_ocpp_1_6(
        &mut self,
        transaction_started_at: Option<DateTime<Utc>>,
        ocpp_evse_id: u32,
    ) -> Result<(), OCPP1_6Error> {
        let transaction_id = create_transaction_id();
        let evse = self.data.evse_by_ocpp_id_or_create(ocpp_evse_id);
        let connector = evse.connector_by_ocpp_id(1).ok_or_else(|| {
            OCPP1_6Error::new_internal_str("Connector with ID 1 not found for EVSE")
        })?;
        let transaction_started_at = transaction_started_at.unwrap_or_else(Utc::now);

        let transaction = self
            .data_store
            .create_transaction(
                &self.id,
                evse.id,
                &transaction_id.to_string(),
                transaction_started_at,
                false,
            )
            .await
            .map_err(|e| OCPP1_6Error::new_internal(&e))?;

        self.event_manager
            .send_event(EventPayload::TransactionStartedEvent(
                TransactionStartedEvent {
                    charger_id: self.id.to_string(),
                    evse_id: evse.id,
                    connector_id: connector.id,
                    transaction_id: transaction.id,
                    authenticated: !self.data.settings.authorize_transactions,
                    started_at: transaction_started_at,
                },
            ))
            .await;

        Ok(())
    }

    pub async fn stop_transaction_ocpp_1_6(
        &self,
        ocpp_transaction_id: i32,
        stopped_at: DateTime<Utc>,
    ) -> Result<(), OCPP1_6Error> {
        let transaction = self
            .data_store
            .end_transaction(&self.id, &ocpp_transaction_id.to_string(), stopped_at)
            .await
            .map_err(|e| OCPP1_6Error::new_internal(&e))?
            .ok_or_else(|| OCPP1_6Error::new_internal_str("Transaction not found"))?;

        self.event_manager
            .send_event(EventPayload::TransactionStoppedEvent(
                TransactionStoppedEvent {
                    charger_id: self.id.to_string(),
                    evse_id: transaction.evse_id,
                    connector_id: Default::default(),
                    transaction_id: transaction.id,
                    started_at: transaction.start_time,
                    stopped_at,
                },
            ))
            .await;

        Ok(())
    }

    pub async fn end_ongoing_transaction_ocpp_1_6(
        &mut self,
        evse_ocpp_id: u32,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<(), OCPP1_6Error> {
        if let Some(transaction) = self
            .get_ongoing_transaction(evse_ocpp_id)
            .await
            .map_err(|e| OCPP1_6Error::new_internal(&e))?
        {
            self.stop_transaction_ocpp_1_6(
                transaction.ocpp_transaction_id.parse().unwrap_or(0),
                end_time.unwrap_or(Utc::now()),
            )
            .await?;
        } else {
            warn!(
                charger_id = self.id,
                evse_ocpp_id, "no ongoing transaction found for EVSE with OCPP ID"
            );
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl Ocpp16RequestReceiver for Charger {
    async fn post_request(
        &mut self,
        action: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.ping().await;
        if action == "BootNotification" {
            match self
                .handle
                .as_ocpp1_6()
                .unwrap()
                .send_get_configuration(GetConfigurationRequest { key: None })
                .await?
            {
                Ok(configuration) => {
                    self.data.ocpp1_6configuration = Some(
                        Ocpp1_6Configuration::from_full_get_configuration_response(&configuration),
                    );

                    if let Some(conf) = &self.data.ocpp1_6configuration {
                        let num_outlets_raw = conf
                            .get_configuration("NumberOfConnectors")
                            .and_then(|i| i.value.clone())
                            .unwrap_or("1".to_string());
                        let num_outlets = num_outlets_raw.parse::<usize>()?;
                        if self.data.evses.len() < num_outlets {
                            for index in 1..=num_outlets {
                                if !self
                                    .data
                                    .evses
                                    .iter()
                                    .any(|i| i.ocpp_evse_id == index as u32)
                                {
                                    self.data.evses.push(EvseData {
                                        id: Uuid::new_v4(),
                                        ocpp_evse_id: index as u32,
                                        connectors: vec![ConnectorData {
                                            id: Uuid::new_v4(),
                                            ocpp_id: 1,
                                            status: ConnectorStatus::Available,
                                            connector_type: ConnectorType::Unknown,
                                        }],
                                        watt_output: Default::default(),
                                        ampere_output: Default::default(),
                                        voltage: Default::default(),
                                    });
                                }
                            }
                        }
                    }

                    if let Err(err) = self.sync_data().await {
                        error!(
                            error_message = err.to_string(),
                            "Failed to update charger database"
                        );
                    }

                    self.update_ocpp_1_6_charger_configuration().await?;
                }
                Err(err) => {
                    warn!(
                        error_message = err.to_string(),
                        "Failed to get response for GetConfigurationRequest request"
                    )
                }
            }
            if !self.authenticated {
                if let Some(ChargerModel::Easee(_)) = self.model() {
                    warn!("Easee chargers uses a master password, it will be matched during next reconnect");
                    self.handle.disconnect().await?;
                } else {
                    info!("Generating new password for charger");
                    let password: String = rand::rng()
                        .sample_iter(&Alphanumeric)
                        .take(20)
                        .map(char::from)
                        .collect();

                    let hex = hex::encode(password.clone());

                    match self
                        .handle
                        .as_ocpp1_6()
                        .unwrap()
                        .send_change_configuration(ChangeConfigurationRequest {
                            key: "AuthorizationKey".to_string(),
                            value: hex.to_string(),
                        })
                        .await?
                    {
                        Ok(result) => {
                            if result.status == ConfigurationStatus::Accepted {
                                let hashed = bcrypt::hash(&password, DEFAULT_COST)?;
                                self.data_store.save_password(&self.id, &hashed).await?;
                                self.handle.disconnect().await?;
                            } else {
                                warn!("Failed to change the AuthorizationKey config value")
                            }
                        }
                        Err(err) => {
                            warn!(
                                error_message = err.to_string(),
                                "Failed to change the AuthorizationKey config value"
                            )
                        }
                    }
                }
            }
        }

        if action == "StatusNotification" && self.data.serial_number.is_none() {
            match self
                .handle
                .as_ocpp1_6()
                .unwrap()
                .send_trigger_message(TriggerMessageRequest {
                    requested_message: MessageTrigger::BootNotification,
                    connector_id: None,
                })
                .await?
            {
                Ok(response) => {
                    if response.status == TriggerMessageStatus::Rejected {
                        // Let's resort to rebooting the charger
                        match self
                            .handle
                            .as_ocpp1_6()
                            .unwrap()
                            .send_reset(ResetRequest {
                                kind: ResetRequestStatus::Soft,
                            })
                            .await?
                        {
                            Ok(response) => {
                                if response.status == ResetResponseStatus::Rejected {
                                    warn!("Failed to restart the charger")
                                }
                            }
                            Err(err) => {
                                warn!(
                                    error_message = err.to_string(),
                                    "Failed to restart the charger"
                                )
                            }
                        }
                    }
                }
                Err(err) => {
                    warn!(
                        error_message = err.to_string(),
                        "Failed to trigger the boot notification"
                    )
                }
            }
        }

        Ok(())
    }

    async fn authorize(
        &mut self,
        request: AuthorizeRequest,
    ) -> Result<AuthorizeResponse, OCPP1_6Error> {
        Ok(AuthorizeResponse {
            id_tag_info: self.validate_rfid_tag_ocpp_1_6(&request.id_tag).await?,
        })
    }

    async fn boot_notification(
        &mut self,
        request: BootNotificationRequest,
    ) -> Result<BootNotificationResponse, OCPP1_6Error> {
        self.data.vendor = Some(request.charge_point_vendor.clone());
        self.data.serial_number = request.charge_point_serial_number;
        self.data.firmware_version = request.firmware_version;
        self.data.iccid = request.iccid;
        self.data.imsi = request.imsi;
        self.data.model = Some(request.charge_point_model);

        if let Err(err) = self.sync_data().await {
            error!(
                error_message = err.to_string(),
                "Failed to update charger database"
            );
        }

        if self.authenticated {
            Ok(BootNotificationResponse {
                current_time: Utc::now(),
                interval: 3600,
                status: RegistrationStatus::Accepted,
            })
        } else {
            Ok(BootNotificationResponse {
                current_time: Utc::now(),
                interval: 5,
                status: RegistrationStatus::Pending,
            })
        }
    }

    async fn data_transfer(
        &mut self,
        _request: DataTransferRequest,
    ) -> Result<DataTransferResponse, OCPP1_6Error> {
        Ok(DataTransferResponse {
            status: DataTransferStatus::Rejected,
            data: None,
        })
    }

    async fn diagnostics_status_notification(
        &mut self,
        _request: DiagnosticsStatusNotificationRequest,
    ) -> Result<DiagnosticsStatusNotificationResponse, OCPP1_6Error> {
        Ok(DiagnosticsStatusNotificationResponse {})
    }

    async fn firmware_status_notification(
        &mut self,
        _request: FirmwareStatusNotificationRequest,
    ) -> Result<FirmwareStatusNotificationResponse, OCPP1_6Error> {
        Ok(FirmwareStatusNotificationResponse {})
    }

    async fn heartbeat(
        &mut self,
        _request: HeartbeatRequest,
    ) -> Result<HeartbeatResponse, OCPP1_6Error> {
        Ok(HeartbeatResponse {
            current_time: Utc::now(),
        })
    }

    async fn meter_values(
        &mut self,
        request: MeterValuesRequest,
    ) -> Result<MeterValuesResponse, OCPP1_6Error> {
        update_charger_from_meter_values_request(&mut self.data, &request)?;
        self.sync_data().await.map_err(|error| {
            error!(
                error_message = error.to_string(),
                "Failed to update charger database"
            );
            OCPP1_6Error::new_internal(&error)
        })?;

        self.send_meter_values_event_ocpp_1_6(&request).await?;

        Ok(MeterValuesResponse {})
    }

    async fn start_transaction(
        &mut self,
        request: StartTransactionRequest,
    ) -> Result<StartTransactionResponse, OCPP1_6Error> {
        let tag = request.id_tag;

        let transaction = self
            .get_ongoing_transaction(request.connector_id)
            .await
            .map_err(|_error| {
                OCPP1_6Error::new_internal_str("Could not get transaction from database")
            })?;

        if let Some(transaction) = &transaction {
            let meter_start = request.meter_start;
            self.data_store
                .update_transaction_meter_start(transaction.id, meter_start)
                .await
                .map_err(|e| {
                    error!(
                        error_message = e.to_string(),
                        "Failed to update transaction"
                    );
                    OCPP1_6Error::new_internal_str("Failed to update transaction")
                })?;
        }

        let transaction_id = transaction
            .map(|t| t.ocpp_transaction_id.parse::<i32>())
            .unwrap_or(Ok(0))
            .map_err(|e| {
                error!(
                    error_message = e.to_string(),
                    "Failed to parse transaction id"
                );
                OCPP1_6Error::new_internal_str("Failed to parse transaction id")
            })?;

        Ok(StartTransactionResponse {
            id_tag_info: self.validate_rfid_tag_ocpp_1_6(&tag).await?,
            transaction_id,
        })
    }

    async fn status_notification(
        &mut self,
        request: StatusNotificationRequest,
    ) -> Result<StatusNotificationResponse, OCPP1_6Error> {
        if request.connector_id != 0 {
            if let Some(connector) = self.data.ocpp_1_6_get_connector(request.connector_id) {
                connector.status = request.status.clone().into();
            }

            if request.status == ChargePointStatus::Available {
                self.end_ongoing_transaction(request.connector_id, &request.timestamp)
                    .await
                    .map_err(|e| OCPP1_6Error::new_internal(&e))?;

                self.end_ongoing_transaction_ocpp_1_6(request.connector_id, request.timestamp)
                    .await?;
            }

            if request.status == ChargePointStatus::Faulted {
                self.end_ongoing_transaction_ocpp_1_6(request.connector_id, request.timestamp)
                    .await?;
            }

            if request.status == ChargePointStatus::Preparing {
                self.start_transaction_ocpp_1_6(request.timestamp, request.connector_id)
                    .await?;
            }

            if let Some(evse) = self.data.evse_by_ocpp_id_mut(request.connector_id) {
                let evse_id = evse.id;
                let evse_ocpp_id = evse.ocpp_evse_id;
                if let Some(connector) = evse.connector_by_ocpp_id_mut(1) {
                    let event_manager = self.event_manager.clone();
                    let charger_id = self.id.clone();
                    let connector_id = connector.id;
                    let timestamp = request.timestamp.unwrap_or_else(Utc::now);
                    if let Some(transaction) = self
                        .get_ongoing_transaction(evse_ocpp_id)
                        .await
                        .map_err(|e| OCPP1_6Error::new_internal(&e))?
                    {
                        event_manager
                            .send_event(EventPayload::TransactionEvent(TransactionEvent {
                                charger_id,
                                timestamp,
                                event_type: TransactionEventType::Updated,
                                trigger_reason: TransactionEventTriggerReason::ChargingStateChanged,
                                number_of_phases_used: None,
                                cable_max_current: None,
                                reservation_id: None,
                                transaction_info: TransactionInfo {
                                    id: transaction.id,
                                    charging_state: None,
                                    time_spent_charging: None,
                                    stopped_reason: None,
                                },
                                evse: EvseInfo {
                                    id: evse_id,
                                    connector_id,
                                },
                                meter_values: vec![],
                            }))
                            .await
                    }
                }
            }
            if let Err(err) = self.sync_data().await {
                error!(
                    error_message = err.to_string(),
                    "Failed to update charger database"
                );
            }
        }
        Ok(StatusNotificationResponse {})
    }

    async fn stop_transaction(
        &mut self,
        request: StopTransactionRequest,
    ) -> Result<StopTransactionResponse, OCPP1_6Error> {
        self.data_store
            .update_transaction_watt_charged(
                &self.id,
                &request.transaction_id.to_string(),
                request.meter_stop,
            )
            .await
            .map_err(|e| OCPP1_6Error::new_internal(&e))?;

        if let Some(tag) = request.id_tag {
            Ok(StopTransactionResponse {
                id_tag_info: Some(self.validate_rfid_tag_ocpp_1_6(&tag).await?),
            })
        } else {
            Ok(StopTransactionResponse { id_tag_info: None })
        }
    }
}

impl From<authorize_response::AuthorizationStatus> for AuthorizationStatus {
    fn from(status: authorize_response::AuthorizationStatus) -> Self {
        match status {
            authorize_response::AuthorizationStatus::Accepted => AuthorizationStatus::Accepted,
            authorize_response::AuthorizationStatus::Blocked => AuthorizationStatus::Blocked,
            authorize_response::AuthorizationStatus::Expired => AuthorizationStatus::Expired,
            authorize_response::AuthorizationStatus::ConcurrentTransaction => {
                AuthorizationStatus::ConcurrentTx
            }
            authorize_response::AuthorizationStatus::Invalid => AuthorizationStatus::Invalid,
            authorize_response::AuthorizationStatus::NoCredit => AuthorizationStatus::Blocked,
            authorize_response::AuthorizationStatus::Unspecified => AuthorizationStatus::Invalid,
            authorize_response::AuthorizationStatus::NotAllowedOnThisTypeOfEvse => {
                AuthorizationStatus::Blocked
            }
            authorize_response::AuthorizationStatus::NotAtThisLocation => {
                AuthorizationStatus::Blocked
            }
            authorize_response::AuthorizationStatus::NotAtThisTime => AuthorizationStatus::Blocked,
            authorize_response::AuthorizationStatus::Unknown => AuthorizationStatus::Invalid,
        }
    }
}
