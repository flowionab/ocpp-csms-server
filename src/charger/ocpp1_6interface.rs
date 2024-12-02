use chrono::Utc;
use ocpp_client::ocpp_1_6::OCPP1_6Error;
use rust_ocpp::v1_6::messages::authorize::{AuthorizeRequest, AuthorizeResponse};
use rust_ocpp::v1_6::messages::boot_notification::{BootNotificationRequest, BootNotificationResponse};
use rust_ocpp::v1_6::messages::data_transfer::{DataTransferRequest, DataTransferResponse};
use rust_ocpp::v1_6::messages::diagnostics_status_notification::{DiagnosticsStatusNotificationRequest, DiagnosticsStatusNotificationResponse};
use rust_ocpp::v1_6::messages::firmware_status_notification::{FirmwareStatusNotificationRequest, FirmwareStatusNotificationResponse};
use rust_ocpp::v1_6::messages::heart_beat::{HeartbeatRequest, HeartbeatResponse};
use rust_ocpp::v1_6::messages::meter_values::{MeterValuesRequest, MeterValuesResponse};
use rust_ocpp::v1_6::messages::start_transaction::{StartTransactionRequest, StartTransactionResponse};
use rust_ocpp::v1_6::messages::status_notification::{StatusNotificationRequest, StatusNotificationResponse};
use rust_ocpp::v1_6::messages::stop_transaction::{StopTransactionRequest, StopTransactionResponse};
use rust_ocpp::v1_6::types::RegistrationStatus;
use crate::charger::charger::Charger;

pub struct Ocpp1_6Interface<'a>{
    pub charger: &'a mut Charger
}

impl<'a> Ocpp1_6Interface<'a> {
    pub fn new(charger: &'a mut Charger) -> Self {
        Self {
            charger
        }
    }

    pub async fn handle_authorize(&mut self, request: AuthorizeRequest) -> Result<AuthorizeResponse, OCPP1_6Error> {
        Err(OCPP1_6Error::new_not_implemented("This action is not implemented"))
    }

    pub async fn handle_boot_notification(&mut self, request: BootNotificationRequest) -> Result<BootNotificationResponse, OCPP1_6Error> {
        self.charger.vendor = Some(request.charge_point_vendor);

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

    pub async fn handle_data_transfer(&mut self, request: DataTransferRequest) -> Result<DataTransferResponse, OCPP1_6Error> {
        Err(OCPP1_6Error::new_not_implemented("This action is not implemented"))
    }

    pub async fn handle_diagnostics_status_notification(&mut self, request: DiagnosticsStatusNotificationRequest) -> Result<DiagnosticsStatusNotificationResponse, OCPP1_6Error> {
        Err(OCPP1_6Error::new_not_implemented("This action is not implemented"))
    }

    pub async fn handle_firmware_status_notification(&mut self, request: FirmwareStatusNotificationRequest) -> Result<FirmwareStatusNotificationResponse, OCPP1_6Error> {
        Err(OCPP1_6Error::new_not_implemented("This action is not implemented"))
    }

    pub async fn handle_heartbeat(&mut self, _request: HeartbeatRequest) -> Result<HeartbeatResponse, OCPP1_6Error> {
        Ok(HeartbeatResponse { current_time: Utc::now() })
    }

    pub async fn handle_meter_values(&mut self, request: MeterValuesRequest) -> Result<MeterValuesResponse, OCPP1_6Error> {
        Err(OCPP1_6Error::new_not_implemented("This action is not implemented"))
    }

    pub async fn handle_start_transaction(&mut self, request: StartTransactionRequest) -> Result<StartTransactionResponse, OCPP1_6Error> {
        Err(OCPP1_6Error::new_not_implemented("This action is not implemented"))
    }

    pub async fn handle_status_notification(&mut self, request: StatusNotificationRequest) -> Result<StatusNotificationResponse, OCPP1_6Error> {
        Err(OCPP1_6Error::new_not_implemented("This action is not implemented"))
    }

    pub async fn handle_stop_transaction(&mut self, request: StopTransactionRequest) -> Result<StopTransactionResponse, OCPP1_6Error> {
        Err(OCPP1_6Error::new_not_implemented("This action is not implemented"))
    }
}