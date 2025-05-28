use ocpp_client::ocpp_2_0_1::OCPP2_0_1Error;
use rust_ocpp::v2_0_1::messages::authorize::{AuthorizeRequest, AuthorizeResponse};
use rust_ocpp::v2_0_1::messages::boot_notification::{
    BootNotificationRequest, BootNotificationResponse,
};
use rust_ocpp::v2_0_1::messages::cleared_charging_limit::{
    ClearedChargingLimitRequest, ClearedChargingLimitResponse,
};
use rust_ocpp::v2_0_1::messages::datatransfer::{DataTransferRequest, DataTransferResponse};
use rust_ocpp::v2_0_1::messages::firmware_status_notification::{
    FirmwareStatusNotificationRequest, FirmwareStatusNotificationResponse,
};
use rust_ocpp::v2_0_1::messages::get_15118ev_certificate::{
    Get15118EVCertificateRequest, Get15118EVCertificateResponse,
};
use rust_ocpp::v2_0_1::messages::get_certificate_status::{
    GetCertificateStatusRequest, GetCertificateStatusResponse,
};
use rust_ocpp::v2_0_1::messages::heartbeat::{HeartbeatRequest, HeartbeatResponse};
use rust_ocpp::v2_0_1::messages::log_status_notification::{
    LogStatusNotificationRequest, LogStatusNotificationResponse,
};
use rust_ocpp::v2_0_1::messages::meter_values::{MeterValuesRequest, MeterValuesResponse};
use rust_ocpp::v2_0_1::messages::notify_customer_information::{
    NotifyCustomerInformationRequest, NotifyCustomerInformationResponse,
};
use rust_ocpp::v2_0_1::messages::notify_display_messages::{
    NotifyDisplayMessagesRequest, NotifyDisplayMessagesResponse,
};
use rust_ocpp::v2_0_1::messages::notify_ev_charging_needs::{
    NotifyEVChargingNeedsRequest, NotifyEVChargingNeedsResponse,
};
use rust_ocpp::v2_0_1::messages::notify_ev_charging_schedule::{
    NotifyEVChargingScheduleRequest, NotifyEVChargingScheduleResponse,
};
use rust_ocpp::v2_0_1::messages::notify_event::{NotifyEventRequest, NotifyEventResponse};
use rust_ocpp::v2_0_1::messages::notify_monitoring_report::{
    NotifyMonitoringReportRequest, NotifyMonitoringReportResponse,
};
use rust_ocpp::v2_0_1::messages::notify_report::{NotifyReportRequest, NotifyReportResponse};
use rust_ocpp::v2_0_1::messages::publish_firmware_status_notification::{
    PublishFirmwareStatusNotificationRequest, PublishFirmwareStatusNotificationResponse,
};
use rust_ocpp::v2_0_1::messages::report_charging_profiles::{
    ReportChargingProfilesRequest, ReportChargingProfilesResponse,
};
use rust_ocpp::v2_0_1::messages::request_start_transaction::{
    RequestStartTransactionRequest, RequestStartTransactionResponse,
};
use rust_ocpp::v2_0_1::messages::request_stop_transaction::{
    RequestStopTransactionRequest, RequestStopTransactionResponse,
};
use rust_ocpp::v2_0_1::messages::reservation_status_update::{
    ReservationStatusUpdateRequest, ReservationStatusUpdateResponse,
};
use rust_ocpp::v2_0_1::messages::security_event_notification::{
    SecurityEventNotificationRequest, SecurityEventNotificationResponse,
};
use rust_ocpp::v2_0_1::messages::sign_certificate::{
    SignCertificateRequest, SignCertificateResponse,
};
use rust_ocpp::v2_0_1::messages::status_notification::{
    StatusNotificationRequest, StatusNotificationResponse,
};
use rust_ocpp::v2_0_1::messages::transaction_event::{
    TransactionEventRequest, TransactionEventResponse,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait Ocpp2_0_1RequestReceiver {
    async fn post_request(
        &mut self,
        action: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;

    async fn authorize(
        &mut self,
        request: AuthorizeRequest,
    ) -> Result<AuthorizeResponse, OCPP2_0_1Error>;

    async fn boot_notification(
        &mut self,
        request: BootNotificationRequest,
    ) -> Result<BootNotificationResponse, OCPP2_0_1Error>;

    async fn cleared_charging_limit(
        &mut self,
        request: ClearedChargingLimitRequest,
    ) -> Result<ClearedChargingLimitResponse, OCPP2_0_1Error>;

    async fn data_transfer(
        &mut self,
        request: DataTransferRequest,
    ) -> Result<DataTransferResponse, OCPP2_0_1Error>;

    async fn firmware_status_notification(
        &mut self,
        request: FirmwareStatusNotificationRequest,
    ) -> Result<FirmwareStatusNotificationResponse, OCPP2_0_1Error>;

    async fn get_15118_ev_certificate(
        &mut self,
        request: Get15118EVCertificateRequest,
    ) -> Result<Get15118EVCertificateResponse, OCPP2_0_1Error>;

    async fn get_certificate_status(
        &mut self,
        request: GetCertificateStatusRequest,
    ) -> Result<GetCertificateStatusResponse, OCPP2_0_1Error>;

    async fn heartbeat(
        &mut self,
        request: HeartbeatRequest,
    ) -> Result<HeartbeatResponse, OCPP2_0_1Error>;

    async fn log_status_notification(
        &mut self,
        request: LogStatusNotificationRequest,
    ) -> Result<LogStatusNotificationResponse, OCPP2_0_1Error>;

    async fn meter_values(
        &mut self,
        request: MeterValuesRequest,
    ) -> Result<MeterValuesResponse, OCPP2_0_1Error>;

    async fn notify_customer_information(
        &mut self,
        request: NotifyCustomerInformationRequest,
    ) -> Result<NotifyCustomerInformationResponse, OCPP2_0_1Error>;

    async fn notify_display_messages(
        &mut self,
        request: NotifyDisplayMessagesRequest,
    ) -> Result<NotifyDisplayMessagesResponse, OCPP2_0_1Error>;

    async fn notify_ev_charging_needs(
        &mut self,
        request: NotifyEVChargingNeedsRequest,
    ) -> Result<NotifyEVChargingNeedsResponse, OCPP2_0_1Error>;

    async fn notify_ev_charging_schedule(
        &mut self,
        request: NotifyEVChargingScheduleRequest,
    ) -> Result<NotifyEVChargingScheduleResponse, OCPP2_0_1Error>;

    async fn notify_event(
        &mut self,
        request: NotifyEventRequest,
    ) -> Result<NotifyEventResponse, OCPP2_0_1Error>;

    async fn notify_monitoring_report(
        &mut self,
        request: NotifyMonitoringReportRequest,
    ) -> Result<NotifyMonitoringReportResponse, OCPP2_0_1Error>;

    async fn notify_report(
        &mut self,
        request: NotifyReportRequest,
    ) -> Result<NotifyReportResponse, OCPP2_0_1Error>;

    async fn publish_firmware_status_notification(
        &mut self,
        request: PublishFirmwareStatusNotificationRequest,
    ) -> Result<PublishFirmwareStatusNotificationResponse, OCPP2_0_1Error>;

    async fn report_charging_profiles(
        &mut self,
        request: ReportChargingProfilesRequest,
    ) -> Result<ReportChargingProfilesResponse, OCPP2_0_1Error>;

    async fn request_start_transaction(
        &mut self,
        request: RequestStartTransactionRequest,
    ) -> Result<RequestStartTransactionResponse, OCPP2_0_1Error>;

    async fn request_stop_transaction(
        &mut self,
        request: RequestStopTransactionRequest,
    ) -> Result<RequestStopTransactionResponse, OCPP2_0_1Error>;

    async fn reservation_status_update(
        &mut self,
        request: ReservationStatusUpdateRequest,
    ) -> Result<ReservationStatusUpdateResponse, OCPP2_0_1Error>;

    async fn security_event_notification(
        &mut self,
        request: SecurityEventNotificationRequest,
    ) -> Result<SecurityEventNotificationResponse, OCPP2_0_1Error>;

    async fn sign_certificate(
        &mut self,
        request: SignCertificateRequest,
    ) -> Result<SignCertificateResponse, OCPP2_0_1Error>;

    async fn status_notification(
        &mut self,
        request: StatusNotificationRequest,
    ) -> Result<StatusNotificationResponse, OCPP2_0_1Error>;

    async fn transaction_event(
        &mut self,
        request: TransactionEventRequest,
    ) -> Result<TransactionEventResponse, OCPP2_0_1Error>;
}
