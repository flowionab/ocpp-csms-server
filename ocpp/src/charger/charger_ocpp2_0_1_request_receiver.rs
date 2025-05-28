use crate::charger::Charger;
use crate::network_interface::Ocpp2_0_1RequestReceiver;
use chrono::Utc;
use ocpp_client::ocpp_2_0_1::OCPP2_0_1Error;
use rust_ocpp::v2_0_1::enumerations::data_transfer_status_enum_type::DataTransferStatusEnumType;
use rust_ocpp::v2_0_1::enumerations::registration_status_enum_type::RegistrationStatusEnumType;
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
use std::error::Error;
use tracing::error;

#[async_trait::async_trait]
impl Ocpp2_0_1RequestReceiver for Charger {
    async fn post_request(
        &mut self,
        _action: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        Ok(())
    }

    async fn authorize(
        &mut self,
        _request: AuthorizeRequest,
    ) -> Result<AuthorizeResponse, OCPP2_0_1Error> {
        todo!()
    }

    async fn boot_notification(
        &mut self,
        request: BootNotificationRequest,
    ) -> Result<BootNotificationResponse, OCPP2_0_1Error> {
        self.data.vendor = Some(request.charging_station.vendor_name.clone());
        self.data.serial_number = request.charging_station.serial_number;
        self.data.firmware_version = request.charging_station.firmware_version;
        self.data.model = Some(request.charging_station.model);

        if let Some(modem) = &request.charging_station.modem {
            self.data.iccid = modem.iccid.clone();
            self.data.imsi = modem.imsi.clone();
        }

        if let Err(err) = self.sync_data().await {
            error!(
                error_message = err.to_string(),
                "Failed to update charger database"
            );
        }

        if self.authenticated {
            Ok(BootNotificationResponse {
                current_time: Utc::now(),
                interval: 30,
                status: RegistrationStatusEnumType::Accepted,
                status_info: None,
            })
        } else {
            Ok(BootNotificationResponse {
                current_time: Utc::now(),
                interval: 5,
                status: RegistrationStatusEnumType::Pending,
                status_info: None,
            })
        }
    }

    async fn cleared_charging_limit(
        &mut self,
        _request: ClearedChargingLimitRequest,
    ) -> Result<ClearedChargingLimitResponse, OCPP2_0_1Error> {
        Ok(ClearedChargingLimitResponse {})
    }

    async fn data_transfer(
        &mut self,
        _request: DataTransferRequest,
    ) -> Result<DataTransferResponse, OCPP2_0_1Error> {
        Ok(DataTransferResponse {
            status: DataTransferStatusEnumType::Rejected,
            data: None,
            status_info: None,
        })
    }

    async fn firmware_status_notification(
        &mut self,
        _request: FirmwareStatusNotificationRequest,
    ) -> Result<FirmwareStatusNotificationResponse, OCPP2_0_1Error> {
        Ok(FirmwareStatusNotificationResponse {})
    }

    async fn get_15118_ev_certificate(
        &mut self,
        _request: Get15118EVCertificateRequest,
    ) -> Result<Get15118EVCertificateResponse, OCPP2_0_1Error> {
        todo!()
    }

    async fn get_certificate_status(
        &mut self,
        _request: GetCertificateStatusRequest,
    ) -> Result<GetCertificateStatusResponse, OCPP2_0_1Error> {
        todo!()
    }

    async fn heartbeat(
        &mut self,
        _request: HeartbeatRequest,
    ) -> Result<HeartbeatResponse, OCPP2_0_1Error> {
        Ok(HeartbeatResponse {
            current_time: Utc::now(),
        })
    }

    async fn log_status_notification(
        &mut self,
        _request: LogStatusNotificationRequest,
    ) -> Result<LogStatusNotificationResponse, OCPP2_0_1Error> {
        Ok(LogStatusNotificationResponse {})
    }

    async fn meter_values(
        &mut self,
        _request: MeterValuesRequest,
    ) -> Result<MeterValuesResponse, OCPP2_0_1Error> {
        Ok(MeterValuesResponse {})
    }

    async fn notify_customer_information(
        &mut self,
        _request: NotifyCustomerInformationRequest,
    ) -> Result<NotifyCustomerInformationResponse, OCPP2_0_1Error> {
        todo!()
    }

    async fn notify_display_messages(
        &mut self,
        _request: NotifyDisplayMessagesRequest,
    ) -> Result<NotifyDisplayMessagesResponse, OCPP2_0_1Error> {
        todo!()
    }

    async fn notify_ev_charging_needs(
        &mut self,
        _request: NotifyEVChargingNeedsRequest,
    ) -> Result<NotifyEVChargingNeedsResponse, OCPP2_0_1Error> {
        todo!()
    }

    async fn notify_ev_charging_schedule(
        &mut self,
        _request: NotifyEVChargingScheduleRequest,
    ) -> Result<NotifyEVChargingScheduleResponse, OCPP2_0_1Error> {
        todo!()
    }

    async fn notify_event(
        &mut self,
        _request: NotifyEventRequest,
    ) -> Result<NotifyEventResponse, OCPP2_0_1Error> {
        todo!()
    }

    async fn notify_monitoring_report(
        &mut self,
        _request: NotifyMonitoringReportRequest,
    ) -> Result<NotifyMonitoringReportResponse, OCPP2_0_1Error> {
        todo!()
    }

    async fn notify_report(
        &mut self,
        _request: NotifyReportRequest,
    ) -> Result<NotifyReportResponse, OCPP2_0_1Error> {
        todo!()
    }

    async fn publish_firmware_status_notification(
        &mut self,
        _request: PublishFirmwareStatusNotificationRequest,
    ) -> Result<PublishFirmwareStatusNotificationResponse, OCPP2_0_1Error> {
        todo!()
    }

    async fn report_charging_profiles(
        &mut self,
        _request: ReportChargingProfilesRequest,
    ) -> Result<ReportChargingProfilesResponse, OCPP2_0_1Error> {
        todo!()
    }

    async fn request_start_transaction(
        &mut self,
        _request: RequestStartTransactionRequest,
    ) -> Result<RequestStartTransactionResponse, OCPP2_0_1Error> {
        todo!()
    }

    async fn request_stop_transaction(
        &mut self,
        _request: RequestStopTransactionRequest,
    ) -> Result<RequestStopTransactionResponse, OCPP2_0_1Error> {
        todo!()
    }

    async fn reservation_status_update(
        &mut self,
        _request: ReservationStatusUpdateRequest,
    ) -> Result<ReservationStatusUpdateResponse, OCPP2_0_1Error> {
        todo!()
    }

    async fn security_event_notification(
        &mut self,
        _request: SecurityEventNotificationRequest,
    ) -> Result<SecurityEventNotificationResponse, OCPP2_0_1Error> {
        todo!()
    }

    async fn sign_certificate(
        &mut self,
        _request: SignCertificateRequest,
    ) -> Result<SignCertificateResponse, OCPP2_0_1Error> {
        todo!()
    }

    async fn status_notification(
        &mut self,
        _request: StatusNotificationRequest,
    ) -> Result<StatusNotificationResponse, OCPP2_0_1Error> {
        todo!()
    }

    async fn transaction_event(
        &mut self,
        _request: TransactionEventRequest,
    ) -> Result<TransactionEventResponse, OCPP2_0_1Error> {
        todo!()
    }
}
