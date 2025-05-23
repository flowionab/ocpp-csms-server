use crate::charger::charger::Charger;
use crate::charger::charger_model::ChargerModel;
use crate::ocpp::OcppProtocol;
use bcrypt::DEFAULT_COST;
use chrono::Utc;
use futures::SinkExt;
use ocpp_client::ocpp_1_6::OCPP1_6Error;
use poem::web::websocket::Message::Text;
use rand::distr::Alphanumeric;
use rand::Rng;
use rust_ocpp::v1_6::messages::authorize::{AuthorizeRequest, AuthorizeResponse};
use rust_ocpp::v1_6::messages::boot_notification::{
    BootNotificationRequest, BootNotificationResponse,
};
use rust_ocpp::v1_6::messages::cancel_reservation::{
    CancelReservationRequest, CancelReservationResponse,
};
use rust_ocpp::v1_6::messages::change_availability::{
    ChangeAvailabilityRequest, ChangeAvailabilityResponse,
};
use rust_ocpp::v1_6::messages::change_configuration::{
    ChangeConfigurationRequest, ChangeConfigurationResponse,
};
use rust_ocpp::v1_6::messages::data_transfer::{DataTransferRequest, DataTransferResponse};
use rust_ocpp::v1_6::messages::diagnostics_status_notification::{
    DiagnosticsStatusNotificationRequest, DiagnosticsStatusNotificationResponse,
};
use rust_ocpp::v1_6::messages::firmware_status_notification::{
    FirmwareStatusNotificationRequest, FirmwareStatusNotificationResponse,
};
use rust_ocpp::v1_6::messages::get_configuration::{
    GetConfigurationRequest, GetConfigurationResponse,
};
use rust_ocpp::v1_6::messages::heart_beat::{HeartbeatRequest, HeartbeatResponse};
use rust_ocpp::v1_6::messages::meter_values::{MeterValuesRequest, MeterValuesResponse};
use rust_ocpp::v1_6::messages::remote_start_transaction::{
    RemoteStartTransactionRequest, RemoteStartTransactionResponse,
};
use rust_ocpp::v1_6::messages::remote_stop_transaction::{
    RemoteStopTransactionRequest, RemoteStopTransactionResponse,
};
use rust_ocpp::v1_6::messages::reset::{ResetRequest, ResetResponse};
use rust_ocpp::v1_6::messages::start_transaction::{
    StartTransactionRequest, StartTransactionResponse,
};
use rust_ocpp::v1_6::messages::status_notification::{
    StatusNotificationRequest, StatusNotificationResponse,
};
use rust_ocpp::v1_6::messages::stop_transaction::{
    StopTransactionRequest, StopTransactionResponse,
};
use rust_ocpp::v1_6::messages::trigger_message::{TriggerMessageRequest, TriggerMessageResponse};
use rust_ocpp::v1_6::types::{
    AuthorizationStatus, ChargePointStatus, ConfigurationStatus, DataTransferStatus, IdTagInfo,
    MessageTrigger, RegistrationStatus, ResetRequestStatus, ResetResponseStatus,
    TriggerMessageStatus,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use shared::{
    ConnectorData, ConnectorStatus, ConnectorType, EvseData, Ocpp1_6Configuration, Transaction,
};
use tokio::sync::oneshot;
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

pub struct Ocpp1_6Interface<'a> {
    pub charger: &'a mut Charger,
}

impl<'a> Ocpp1_6Interface<'a> {
    pub fn new(charger: &'a mut Charger) -> Self {
        Self { charger }
    }

    pub async fn post_request(
        &mut self,
        action: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.charger.ping().await;
        if action == "BootNotification" {
            match self
                .send_get_configuration(GetConfigurationRequest { key: None })
                .await?
            {
                Ok(configuration) => {
                    self.charger.data.ocpp1_6configuration = Some(
                        Ocpp1_6Configuration::from_full_get_configuration_response(&configuration),
                    );

                    if let Some(conf) = &self.charger.data.ocpp1_6configuration {
                        let num_outlets_raw = conf
                            .get_configuration("NumberOfConnectors")
                            .and_then(|i| i.value.clone())
                            .unwrap_or("1".to_string());
                        let num_outlets = num_outlets_raw.parse::<usize>()?;
                        if self.charger.data.evses.len() < num_outlets {
                            for index in 1..=num_outlets {
                                if !self
                                    .charger
                                    .data
                                    .evses
                                    .iter()
                                    .any(|i| i.ocpp_evse_id == index as u32)
                                {
                                    self.charger.data.evses.push(EvseData {
                                        id: Uuid::new_v4(),
                                        ocpp_evse_id: index as u32,
                                        connectors: vec![ConnectorData {
                                            id: Uuid::new_v4(),
                                            ocpp_id: 1,
                                            status: ConnectorStatus::Available,
                                            connector_type: ConnectorType::Unknown,
                                        }],
                                    });
                                }
                            }
                        }
                    }

                    if let Err(err) = self.charger.sync_data().await {
                        error!(
                            error_message = err.to_string(),
                            "Failed to update charger database"
                        );
                    }
                }
                Err(err) => {
                    warn!(
                        error_message = err.to_string(),
                        "Failed to get response for GetConfigurationRequest request"
                    )
                }
            }
            if !self.charger.authenticated {
                if let Some(ChargerModel::Easee(_)) = self.charger.model() {
                    warn!("Easee chargers uses a master password, it will be matched during next reconnect");
                    let mut lock = self.charger.sink.as_ref().unwrap().lock().await;
                    lock.close().await?;
                } else {
                    info!("Generating new password for charger");
                    let password: String = rand::rng()
                        .sample_iter(&Alphanumeric)
                        .take(20)
                        .map(char::from)
                        .collect();

                    let hex = hex::encode(password.clone());

                    match self
                        .send_change_configuration(ChangeConfigurationRequest {
                            key: "AuthorizationKey".to_string(),
                            value: hex.to_string(),
                        })
                        .await?
                    {
                        Ok(result) => {
                            if result.status == ConfigurationStatus::Accepted {
                                let hashed = bcrypt::hash(&password, DEFAULT_COST)?;
                                self.charger
                                    .data_store
                                    .save_password(&self.charger.id, &hashed)
                                    .await?;
                                let mut lock = self.charger.sink.as_ref().unwrap().lock().await;
                                lock.close().await?;
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

        if action == "StatusNotification" && self.charger.data.serial_number.is_none() {
            match self
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

    #[instrument(skip(self))]
    pub async fn send_get_configuration(
        &self,
        request: GetConfigurationRequest,
    ) -> Result<
        Result<GetConfigurationResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        self.send("GetConfiguration", request).await
    }

    #[instrument(skip(self))]
    pub async fn send_change_configuration(
        &self,
        request: ChangeConfigurationRequest,
    ) -> Result<
        Result<ChangeConfigurationResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        self.send("ChangeConfiguration", request).await
    }

    #[instrument(skip(self))]
    pub async fn send_remote_start_transaction(
        &self,
        request: RemoteStartTransactionRequest,
    ) -> Result<
        Result<RemoteStartTransactionResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        self.send("RemoteStartTransaction", request).await
    }

    #[instrument(skip(self))]
    pub async fn send_remote_stop_transaction(
        &self,
        request: RemoteStopTransactionRequest,
    ) -> Result<
        Result<RemoteStopTransactionResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        self.send("RemoteStopTransaction", request).await
    }

    #[instrument(skip(self))]
    pub async fn send_trigger_message(
        &self,
        request: TriggerMessageRequest,
    ) -> Result<
        Result<TriggerMessageResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        self.send("TriggerMessage", request).await
    }

    #[instrument(skip(self))]
    pub async fn send_reset(
        &self,
        request: ResetRequest,
    ) -> Result<
        Result<ResetResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        self.send("Reset", request).await
    }

    #[instrument(skip(self))]
    pub async fn send_cancel_reservation(
        &self,
        request: CancelReservationRequest,
    ) -> Result<
        Result<CancelReservationResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        self.send("CancelReservation", request).await
    }

    #[instrument(skip(self))]
    pub async fn send_change_availability(
        &self,
        request: ChangeAvailabilityRequest,
    ) -> Result<
        Result<ChangeAvailabilityResponse, OCPP1_6Error>,
        Box<dyn std::error::Error + Send + Sync + 'static>,
    > {
        self.send("ChangeAvailability", request).await
    }

    async fn send<T: Serialize, R: DeserializeOwned>(
        &self,
        action: &str,
        request: T,
    ) -> Result<Result<R, OCPP1_6Error>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let sink = self
            .charger
            .sink
            .as_ref()
            .ok_or("Charger not connected yet")?;
        let message_id = Uuid::new_v4();
        let raw_payload = serde_json::to_string(&(2, message_id, action, request))?;

        info!(
            charger_id = self.charger.id,
            protocol = OcppProtocol::Ocpp1_6.to_string(),
            message_id = &message_id.to_string(),
            action = action,
            raw_payload = &raw_payload,
            "Sending call -->"
        );

        {
            let mut lock = sink.lock().await;
            lock.send(Text(raw_payload)).await?;
        }

        let (sender, receiver) = oneshot::channel();

        {
            let mut lock = self.charger.message_queue.lock().await;
            lock.insert(message_id.to_string(), sender);
        }

        info!("Waiting for response");
        match receiver.await? {
            Ok(val) => {
                let result = serde_json::from_value(val)?;
                Ok(Ok(result))
            }
            Err(err) => Ok(Err(err)),
        }
    }

    #[instrument(skip(self))]
    pub async fn handle_authorize(
        &mut self,
        request: AuthorizeRequest,
    ) -> Result<AuthorizeResponse, OCPP1_6Error> {
        let tag = request.id_tag;

        let is_tag_valid = self.validate_tag(&tag).await?;

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

    #[instrument(skip(self))]
    pub async fn handle_boot_notification(
        &mut self,
        request: BootNotificationRequest,
    ) -> Result<BootNotificationResponse, OCPP1_6Error> {
        self.charger.data.vendor = Some(request.charge_point_vendor.clone());
        self.charger.data.serial_number = request.charge_point_serial_number;
        self.charger.data.firmware_version = request.firmware_version;
        self.charger.data.iccid = request.iccid;
        self.charger.data.imsi = request.imsi;
        self.charger.data.model = Some(request.charge_point_model);

        if let Err(err) = self.charger.sync_data().await {
            error!(
                error_message = err.to_string(),
                "Failed to update charger database"
            );
        }

        if self.charger.authenticated {
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

    #[instrument(skip(self))]
    pub async fn handle_data_transfer(
        &mut self,
        _request: DataTransferRequest,
    ) -> Result<DataTransferResponse, OCPP1_6Error> {
        Ok(DataTransferResponse {
            status: DataTransferStatus::Rejected,
            data: None,
        })
    }

    #[instrument(skip(self))]
    pub async fn handle_diagnostics_status_notification(
        &mut self,
        _request: DiagnosticsStatusNotificationRequest,
    ) -> Result<DiagnosticsStatusNotificationResponse, OCPP1_6Error> {
        Ok(DiagnosticsStatusNotificationResponse {})
    }

    #[instrument(skip(self))]
    pub async fn handle_firmware_status_notification(
        &mut self,
        _request: FirmwareStatusNotificationRequest,
    ) -> Result<FirmwareStatusNotificationResponse, OCPP1_6Error> {
        Ok(FirmwareStatusNotificationResponse {})
    }

    #[instrument(skip(self))]
    pub async fn handle_heartbeat(
        &mut self,
        _request: HeartbeatRequest,
    ) -> Result<HeartbeatResponse, OCPP1_6Error> {
        Ok(HeartbeatResponse {
            current_time: Utc::now(),
        })
    }

    #[instrument(skip(self))]
    pub async fn handle_meter_values(
        &mut self,
        _request: MeterValuesRequest,
    ) -> Result<MeterValuesResponse, OCPP1_6Error> {
        Ok(MeterValuesResponse {})
    }

    #[instrument(skip(self))]
    pub async fn handle_start_transaction(
        &mut self,
        request: StartTransactionRequest,
    ) -> Result<StartTransactionResponse, OCPP1_6Error> {
        let tag = request.id_tag;

        let transaction = self.get_ongoing_transaction().await?;

        let is_tag_valid = self.validate_tag(&tag).await?;

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

    #[instrument(skip(self))]
    pub async fn handle_status_notification(
        &mut self,
        request: StatusNotificationRequest,
    ) -> Result<StatusNotificationResponse, OCPP1_6Error> {
        if request.connector_id != 0 {
            match request.status {
                ChargePointStatus::Available => {
                    if let Some(connector) = self.unwrap_connector(request.connector_id) {
                        connector.status = ConnectorStatus::Available;
                    }
                    self.end_ongoing_transaction(&request.timestamp).await?;
                }
                ChargePointStatus::Preparing => {
                    if let Some(connector) = self.unwrap_connector(request.connector_id) {
                        connector.status = ConnectorStatus::Occupied;
                    }

                    let transaction_id: i32 = {
                        let mut rng = rand::rng();
                        rng.random::<i32>()
                    };
                    self.charger
                        .data_store
                        .create_transaction(
                            &self.charger.id,
                            &transaction_id.to_string(),
                            request.timestamp.unwrap_or_else(Utc::now),
                            false,
                        )
                        .await
                        .map_err(|e| OCPP1_6Error::new_internal(&e))?;
                }
                ChargePointStatus::Charging => {
                    if let Some(connector) = self.unwrap_connector(request.connector_id) {
                        connector.status = ConnectorStatus::Occupied;
                    }
                }
                ChargePointStatus::SuspendedEVSE => {
                    if let Some(connector) = self.unwrap_connector(request.connector_id) {
                        connector.status = ConnectorStatus::Occupied;
                    }
                }
                ChargePointStatus::SuspendedEV => {
                    if let Some(connector) = self.unwrap_connector(request.connector_id) {
                        connector.status = ConnectorStatus::Occupied;
                    }
                }
                ChargePointStatus::Finishing => {
                    if let Some(connector) = self.unwrap_connector(request.connector_id) {
                        connector.status = ConnectorStatus::Occupied;
                    }
                }
                ChargePointStatus::Reserved => {
                    if let Some(connector) = self.unwrap_connector(request.connector_id) {
                        connector.status = ConnectorStatus::Reserved;
                    }
                }
                ChargePointStatus::Unavailable => {
                    if let Some(connector) = self.unwrap_connector(request.connector_id) {
                        connector.status = ConnectorStatus::Unavailable;
                    }
                }
                ChargePointStatus::Faulted => {
                    if let Some(connector) = self.unwrap_connector(request.connector_id) {
                        connector.status = ConnectorStatus::Faulted;
                    }
                }
            }
            if let Some(evse) = self.charger.data.evse_by_ocpp_id_mut(request.connector_id) {
                let evse_id = evse.id;
                if let Some(connector) = evse.connector_by_ocpp_id_mut(1) {
                    self.charger
                        .event_manager
                        .send_connector_status_event(
                            self.charger.id.clone(),
                            request.status.into(),
                            request.timestamp.unwrap_or_else(Utc::now),
                            evse_id,
                            connector.id,
                        )
                        .await;
                }
            }
            if let Err(err) = self.charger.sync_data().await {
                error!(
                    error_message = err.to_string(),
                    "Failed to update charger database"
                );
            }
        }
        Ok(StatusNotificationResponse {})
    }

    #[instrument(skip(self))]
    pub async fn handle_stop_transaction(
        &mut self,
        request: StopTransactionRequest,
    ) -> Result<StopTransactionResponse, OCPP1_6Error> {
        self.charger
            .data_store
            .update_transaction_watt_charged(
                &self.charger.id,
                &request.transaction_id.to_string(),
                request.meter_stop,
            )
            .await
            .map_err(|e| OCPP1_6Error::new_internal(&e))?;

        self.charger
            .data_store
            .end_transaction(
                &self.charger.id,
                &request.transaction_id.to_string(),
                request.timestamp,
            )
            .await
            .map_err(|e| OCPP1_6Error::new_internal(&e))?;

        if let Some(tag) = request.id_tag {
            let is_tag_valid = self.validate_tag(&tag).await?;

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

    async fn get_ongoing_transaction(&mut self) -> Result<Option<Transaction>, OCPP1_6Error> {
        self.charger
            .data_store
            .get_ongoing_transaction(&self.charger.id)
            .await
            .map_err(|e| OCPP1_6Error::new_internal(&e))
    }

    async fn end_ongoing_transaction(
        &mut self,
        end_time: &Option<chrono::DateTime<Utc>>,
    ) -> Result<(), OCPP1_6Error> {
        if let Some(transaction) = self.get_ongoing_transaction().await? {
            self.charger
                .data_store
                .end_transaction(
                    &self.charger.id,
                    &transaction.ocpp_transaction_id,
                    end_time.unwrap_or_else(Utc::now),
                )
                .await
                .map_err(|e| OCPP1_6Error::new_internal(&e))?
        }

        Ok(())
    }

    fn unwrap_connector(&mut self, connector_id: u32) -> Option<&mut ConnectorData> {
        self.charger
            .data
            .evse_by_ocpp_id_mut(connector_id)
            .and_then(|evse| evse.connector_by_ocpp_id_mut(1))
    }

    async fn validate_tag(&self, tag: &str) -> Result<bool, OCPP1_6Error> {
        self.charger.validate_rfid_tag(tag).await.map_err(|error| {
            error!(
                error_message = error.to_string(),
                rfid_tag = tag,
                "Failed to retrieve rfid tag from database"
            );
            OCPP1_6Error::new_internal_str("Could not validate the tag against our database")
        })
    }
}
