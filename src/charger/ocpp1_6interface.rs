use chrono::Utc;
use futures::SinkExt;
use ocpp_client::ocpp_1_6::OCPP1_6Error;
use poem::web::websocket::Message::Text;
use rust_ocpp::v1_6::messages::authorize::{AuthorizeRequest, AuthorizeResponse};
use rust_ocpp::v1_6::messages::boot_notification::{BootNotificationRequest, BootNotificationResponse};
use rust_ocpp::v1_6::messages::change_configuration::{ChangeConfigurationRequest, ChangeConfigurationResponse};
use rust_ocpp::v1_6::messages::data_transfer::{DataTransferRequest, DataTransferResponse};
use rust_ocpp::v1_6::messages::diagnostics_status_notification::{DiagnosticsStatusNotificationRequest, DiagnosticsStatusNotificationResponse};
use rust_ocpp::v1_6::messages::firmware_status_notification::{FirmwareStatusNotificationRequest, FirmwareStatusNotificationResponse};
use rust_ocpp::v1_6::messages::get_configuration::{GetConfigurationRequest, GetConfigurationResponse};
use rust_ocpp::v1_6::messages::heart_beat::{HeartbeatRequest, HeartbeatResponse};
use rust_ocpp::v1_6::messages::meter_values::{MeterValuesRequest, MeterValuesResponse};
use rust_ocpp::v1_6::messages::start_transaction::{StartTransactionRequest, StartTransactionResponse};
use rust_ocpp::v1_6::messages::status_notification::{StatusNotificationRequest, StatusNotificationResponse};
use rust_ocpp::v1_6::messages::stop_transaction::{StopTransactionRequest, StopTransactionResponse};
use rust_ocpp::v1_6::types::RegistrationStatus;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::oneshot;
use tracing::info;
use uuid::Uuid;
use crate::charger::charger::Charger;
use crate::charger::charger_model::ChargerModel;
use crate::ocpp::OcppProtocol;

pub struct Ocpp1_6Interface<'a>{
    pub charger: &'a mut Charger
}

impl<'a> Ocpp1_6Interface<'a> {
    pub fn new(charger: &'a mut Charger) -> Self {
        Self {
            charger
        }
    }

    pub async fn post_request(&mut self, _action: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        if self.charger.authenticated == false {
            match self.send_get_configuration(GetConfigurationRequest {
                key: None,
            }).await? {
                Ok(_) => {
                    match self.send_change_configuration(ChangeConfigurationRequest {
                        key: "".to_string(),
                        value: "".to_string(),
                    }).await? {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                }
                Err(_) => {}
            }
        }

        Ok(())
    }

    pub async fn send_get_configuration(&self, request: GetConfigurationRequest) -> Result<Result<GetConfigurationResponse, OCPP1_6Error>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.send("GetConfiguration", request).await
    }

    pub async fn send_change_configuration(&self, request: ChangeConfigurationRequest) -> Result<Result<ChangeConfigurationResponse, OCPP1_6Error>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.send("ChangeConfiguration", request).await
    }

    async fn send<T: Serialize, R: DeserializeOwned>(&self, action: &str, request: T) -> Result<Result<R, OCPP1_6Error>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let sink = self.charger.sink.as_ref().ok_or_else(|| "Charger not connected yet")?;
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
            Err(err) => {
                Ok(Err(err))
            }
        }
    }

    pub async fn handle_authorize(&mut self, _request: AuthorizeRequest) -> Result<AuthorizeResponse, OCPP1_6Error> {
        Err(OCPP1_6Error::new_not_implemented("This action is not implemented"))
    }

    pub async fn handle_boot_notification(&mut self, request: BootNotificationRequest) -> Result<BootNotificationResponse, OCPP1_6Error> {
        self.charger.vendor = Some(request.charge_point_vendor.clone());
        self.charger.serial_number = request.charge_point_serial_number;
        self.charger.firmware_version = request.firmware_version;
        self.charger.iccid = request.iccid;
        self.charger.imsi = request.imsi;
        self.charger.model = Some(ChargerModel::from_vendor_and_model(&request.charge_point_vendor, &request.charge_point_model));

        if self.charger.authenticated {
            Ok(BootNotificationResponse {
                current_time: Utc::now(),
                interval: 30,
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

    pub async fn handle_data_transfer(&mut self, _request: DataTransferRequest) -> Result<DataTransferResponse, OCPP1_6Error> {
        Err(OCPP1_6Error::new_not_implemented("This action is not implemented"))
    }

    pub async fn handle_diagnostics_status_notification(&mut self, _request: DiagnosticsStatusNotificationRequest) -> Result<DiagnosticsStatusNotificationResponse, OCPP1_6Error> {
        Err(OCPP1_6Error::new_not_implemented("This action is not implemented"))
    }

    pub async fn handle_firmware_status_notification(&mut self, _request: FirmwareStatusNotificationRequest) -> Result<FirmwareStatusNotificationResponse, OCPP1_6Error> {
        Err(OCPP1_6Error::new_not_implemented("This action is not implemented"))
    }

    pub async fn handle_heartbeat(&mut self, _request: HeartbeatRequest) -> Result<HeartbeatResponse, OCPP1_6Error> {
        Ok(HeartbeatResponse { current_time: Utc::now() })
    }

    pub async fn handle_meter_values(&mut self, _request: MeterValuesRequest) -> Result<MeterValuesResponse, OCPP1_6Error> {
        Err(OCPP1_6Error::new_not_implemented("This action is not implemented"))
    }

    pub async fn handle_start_transaction(&mut self, _request: StartTransactionRequest) -> Result<StartTransactionResponse, OCPP1_6Error> {
        Err(OCPP1_6Error::new_not_implemented("This action is not implemented"))
    }

    pub async fn handle_status_notification(&mut self, _request: StatusNotificationRequest) -> Result<StatusNotificationResponse, OCPP1_6Error> {
        Err(OCPP1_6Error::new_not_implemented("This action is not implemented"))
    }

    pub async fn handle_stop_transaction(&mut self, _request: StopTransactionRequest) -> Result<StopTransactionResponse, OCPP1_6Error> {
        Err(OCPP1_6Error::new_not_implemented("This action is not implemented"))
    }
}