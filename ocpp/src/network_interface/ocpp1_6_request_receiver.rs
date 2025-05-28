use ocpp_client::ocpp_1_6::OCPP1_6Error;
use rust_ocpp::v1_6::messages::authorize::{AuthorizeRequest, AuthorizeResponse};
use rust_ocpp::v1_6::messages::boot_notification::{
    BootNotificationRequest, BootNotificationResponse,
};
use rust_ocpp::v1_6::messages::data_transfer::{DataTransferRequest, DataTransferResponse};
use rust_ocpp::v1_6::messages::diagnostics_status_notification::{
    DiagnosticsStatusNotificationRequest, DiagnosticsStatusNotificationResponse,
};
use rust_ocpp::v1_6::messages::firmware_status_notification::{
    FirmwareStatusNotificationRequest, FirmwareStatusNotificationResponse,
};
use rust_ocpp::v1_6::messages::heart_beat::{HeartbeatRequest, HeartbeatResponse};
use rust_ocpp::v1_6::messages::meter_values::{MeterValuesRequest, MeterValuesResponse};
use rust_ocpp::v1_6::messages::start_transaction::{
    StartTransactionRequest, StartTransactionResponse,
};
use rust_ocpp::v1_6::messages::status_notification::{
    StatusNotificationRequest, StatusNotificationResponse,
};
use rust_ocpp::v1_6::messages::stop_transaction::{
    StopTransactionRequest, StopTransactionResponse,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait Ocpp16RequestReceiver {
    async fn post_request(
        &mut self,
        action: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn authorize(
        &mut self,
        request: AuthorizeRequest,
    ) -> Result<AuthorizeResponse, OCPP1_6Error>;

    async fn boot_notification(
        &mut self,
        request: BootNotificationRequest,
    ) -> Result<BootNotificationResponse, OCPP1_6Error>;

    async fn data_transfer(
        &mut self,
        _request: DataTransferRequest,
    ) -> Result<DataTransferResponse, OCPP1_6Error>;

    async fn diagnostics_status_notification(
        &mut self,
        _request: DiagnosticsStatusNotificationRequest,
    ) -> Result<DiagnosticsStatusNotificationResponse, OCPP1_6Error>;

    async fn firmware_status_notification(
        &mut self,
        _request: FirmwareStatusNotificationRequest,
    ) -> Result<FirmwareStatusNotificationResponse, OCPP1_6Error>;

    async fn heartbeat(
        &mut self,
        _request: HeartbeatRequest,
    ) -> Result<HeartbeatResponse, OCPP1_6Error>;

    async fn meter_values(
        &mut self,
        request: MeterValuesRequest,
    ) -> Result<MeterValuesResponse, OCPP1_6Error>;

    async fn start_transaction(
        &mut self,
        request: StartTransactionRequest,
    ) -> Result<StartTransactionResponse, OCPP1_6Error>;

    async fn status_notification(
        &mut self,
        request: StatusNotificationRequest,
    ) -> Result<StatusNotificationResponse, OCPP1_6Error>;

    async fn stop_transaction(
        &mut self,
        request: StopTransactionRequest,
    ) -> Result<StopTransactionResponse, OCPP1_6Error>;
}
