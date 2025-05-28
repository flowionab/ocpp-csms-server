use crate::charger::charger_model::ChargerModel;
use crate::charger::ocpp1_6::handle_meter_values_request;
use crate::charger::Charger;
use crate::network_interface::Ocpp16RequestReceiver;
use bcrypt::DEFAULT_COST;
use chrono::Utc;
use ocpp_client::ocpp_1_6::OCPP1_6Error;
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
    MessageTrigger, RegistrationStatus, ResetRequestStatus, ResetResponseStatus,
    TriggerMessageStatus,
};
use shared::{ConnectorData, ConnectorStatus, ConnectorType, EvseData, Ocpp1_6Configuration};
use std::error::Error;
use tracing::{error, info, warn};
use uuid::Uuid;

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
        let tag = request.id_tag;

        let is_tag_valid = self.validate_rfid_tag(&tag).await.map_err(|_error| {
            OCPP1_6Error::new_internal_str("Could not validate the tag against our database")
        })?;

        if is_tag_valid {
            Ok(AuthorizeResponse {
                id_tag_info: IdTagInfo {
                    expiry_date: None,
                    parent_id_tag: None,
                    status: AuthorizationStatus::Accepted,
                },
            })
        } else {
            Ok(AuthorizeResponse {
                id_tag_info: IdTagInfo {
                    expiry_date: None,
                    parent_id_tag: None,
                    status: AuthorizationStatus::Invalid,
                },
            })
        }
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
        handle_meter_values_request(&mut self.data, request)?;
        self.sync_data().await.map_err(|error| {
            error!(
                error_message = error.to_string(),
                "Failed to update charger database"
            );
            OCPP1_6Error::new_internal(&error)
        })?;
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

        let is_tag_valid = self.validate_rfid_tag(&tag).await.map_err(|_error| {
            OCPP1_6Error::new_internal_str("Could not validate the tag against our database")
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

        if is_tag_valid {
            Ok(StartTransactionResponse {
                id_tag_info: IdTagInfo {
                    expiry_date: None,
                    parent_id_tag: None,
                    status: AuthorizationStatus::Accepted,
                },
                transaction_id,
            })
        } else {
            Ok(StartTransactionResponse {
                id_tag_info: IdTagInfo {
                    expiry_date: None,
                    parent_id_tag: None,
                    status: AuthorizationStatus::Invalid,
                },
                transaction_id,
            })
        }
    }

    async fn status_notification(
        &mut self,
        request: StatusNotificationRequest,
    ) -> Result<StatusNotificationResponse, OCPP1_6Error> {
        if request.connector_id != 0 {
            match request.status {
                ChargePointStatus::Available => {
                    if let Some(connector) = self.data.ocpp_1_6_get_connector(request.connector_id)
                    {
                        connector.status = ConnectorStatus::Available;
                    }
                    self.end_ongoing_transaction(request.connector_id, &request.timestamp)
                        .await
                        .map_err(|e| OCPP1_6Error::new_internal(&e))?;
                }
                ChargePointStatus::Preparing => {
                    if let Some(connector) = self.data.ocpp_1_6_get_connector(request.connector_id)
                    {
                        connector.status = ConnectorStatus::Occupied;
                    }

                    let transaction_id: i32 = {
                        let mut rng = rand::rng();
                        rng.random::<i32>()
                    };
                    let evse = self.data.evse_by_ocpp_id_or_create(request.connector_id);

                    self.data_store
                        .create_transaction(
                            &self.id,
                            evse.id,
                            &transaction_id.to_string(),
                            request.timestamp.unwrap_or_else(Utc::now),
                            false,
                        )
                        .await
                        .map_err(|e| OCPP1_6Error::new_internal(&e))?;
                }
                ChargePointStatus::Charging => {
                    if let Some(connector) = self.data.ocpp_1_6_get_connector(request.connector_id)
                    {
                        connector.status = ConnectorStatus::Occupied;
                    }
                }
                ChargePointStatus::SuspendedEVSE => {
                    if let Some(connector) = self.data.ocpp_1_6_get_connector(request.connector_id)
                    {
                        connector.status = ConnectorStatus::Occupied;
                    }
                }
                ChargePointStatus::SuspendedEV => {
                    if let Some(connector) = self.data.ocpp_1_6_get_connector(request.connector_id)
                    {
                        connector.status = ConnectorStatus::Occupied;
                    }
                }
                ChargePointStatus::Finishing => {
                    if let Some(connector) = self.data.ocpp_1_6_get_connector(request.connector_id)
                    {
                        connector.status = ConnectorStatus::Occupied;
                    }
                }
                ChargePointStatus::Reserved => {
                    if let Some(connector) = self.data.ocpp_1_6_get_connector(request.connector_id)
                    {
                        connector.status = ConnectorStatus::Reserved;
                    }
                }
                ChargePointStatus::Unavailable => {
                    if let Some(connector) = self.data.ocpp_1_6_get_connector(request.connector_id)
                    {
                        connector.status = ConnectorStatus::Unavailable;
                    }
                }
                ChargePointStatus::Faulted => {
                    if let Some(connector) = self.data.ocpp_1_6_get_connector(request.connector_id)
                    {
                        connector.status = ConnectorStatus::Faulted;
                    }
                }
            }
            if let Some(evse) = self.data.evse_by_ocpp_id_mut(request.connector_id) {
                let evse_id = evse.id;
                if let Some(connector) = evse.connector_by_ocpp_id_mut(1) {
                    info!(
                        "Sending connector status event for charger {}: connector {} is now {:?}",
                        self.id, connector.ocpp_id, request.status
                    );
                    let event_manager = self.event_manager.clone();
                    let charger_id = self.id.clone();
                    let connector_id = connector.id;
                    let status = request.status.into();
                    let timestamp = request.timestamp.unwrap_or_else(Utc::now);
                    tokio::spawn(async move {
                        event_manager
                            .send_connector_status_event(
                                charger_id,
                                status,
                                timestamp,
                                evse_id,
                                connector_id,
                            )
                            .await
                    });
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

        self.data_store
            .end_transaction(
                &self.id,
                &request.transaction_id.to_string(),
                request.timestamp,
            )
            .await
            .map_err(|e| OCPP1_6Error::new_internal(&e))?;

        if let Some(tag) = request.id_tag {
            let is_tag_valid = self.validate_rfid_tag(&tag).await.map_err(|_error| {
                OCPP1_6Error::new_internal_str("Could not validate the tag against our database")
            })?;

            if is_tag_valid {
                Ok(StopTransactionResponse {
                    id_tag_info: Some(IdTagInfo {
                        expiry_date: None,
                        parent_id_tag: None,
                        status: AuthorizationStatus::Accepted,
                    }),
                })
            } else {
                Ok(StopTransactionResponse {
                    id_tag_info: Some(IdTagInfo {
                        expiry_date: None,
                        parent_id_tag: None,
                        status: AuthorizationStatus::Invalid,
                    }),
                })
            }
        } else {
            Ok(StopTransactionResponse { id_tag_info: None })
        }
    }
}
