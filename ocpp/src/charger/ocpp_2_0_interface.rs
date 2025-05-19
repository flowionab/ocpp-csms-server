use crate::charger::Charger;
use chrono::Utc;
use ocpp_client::ocpp_2_0_1::OCPP2_0_1Error;
use rust_ocpp::v2_0_1::datatypes::id_token_info_type::IdTokenInfoType;
use rust_ocpp::v2_0_1::enumerations::authorization_status_enum_type::AuthorizationStatusEnumType;
use rust_ocpp::v2_0_1::enumerations::data_transfer_status_enum_type::DataTransferStatusEnumType;
use rust_ocpp::v2_0_1::enumerations::id_token_enum_type::IdTokenEnumType;
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
use tracing::{error, info, instrument};

#[allow(dead_code)]
pub struct Ocpp2_0_1Interface<'a> {
    pub charger: &'a mut Charger,
}

impl<'a> Ocpp2_0_1Interface<'a> {
    pub fn new(charger: &'a mut Charger) -> Self {
        Self { charger }
    }

    #[instrument(skip(self))]
    async fn validate_rfid_token(&self, rfid_token: &str) -> Result<bool, OCPP2_0_1Error> {
        let rfid_tag = self
            .charger
            .data_store
            .get_rfid_tag_by_hex(rfid_token)
            .await
            .map_err(|error| {
                error!(
                    error_message = error.to_string(),
                    rfid_tag = rfid_token,
                    "Failed to retrieve rfid tag from database"
                );
                OCPP2_0_1Error::new_internal_str("Could not validate the tag against our database")
            })?;

        Ok(rfid_tag.is_some())
    }

    #[instrument(skip(self))]
    async fn authorize_rfid_tag(
        &self,
        rfid_token: &str,
    ) -> Result<AuthorizeResponse, OCPP2_0_1Error> {
        if self.validate_rfid_token(rfid_token).await? {
            Ok(AuthorizeResponse {
                certificate_status: None,
                id_token_info: IdTokenInfoType {
                    status: AuthorizationStatusEnumType::Accepted,
                    cache_expiry_date_time: None,
                    charging_priority: None,
                    language1: None,
                    evse_id: None,
                    language2: None,
                    group_id_token: None,
                    personal_message: None,
                },
            })
        } else {
            Ok(AuthorizeResponse {
                certificate_status: None,
                id_token_info: IdTokenInfoType {
                    status: AuthorizationStatusEnumType::Invalid,
                    cache_expiry_date_time: None,
                    charging_priority: None,
                    language1: None,
                    evse_id: None,
                    language2: None,
                    group_id_token: None,
                    personal_message: None,
                },
            })
        }
    }

    #[instrument(skip(self))]
    pub async fn handle_authorize(
        &mut self,
        request: AuthorizeRequest,
    ) -> Result<AuthorizeResponse, OCPP2_0_1Error> {
        info!(
            id_token_kink = format!("{:?}", request.id_token.kind),
            "Received authorize request"
        );
        match request.id_token.kind {
            // An authorize request should never be sent with a central id token
            IdTokenEnumType::Central => Err(OCPP2_0_1Error::new_internal_str(
                "Central authorization is not supported",
            )),
            IdTokenEnumType::EMAID => Ok(AuthorizeResponse {
                certificate_status: None,
                id_token_info: IdTokenInfoType {
                    status: AuthorizationStatusEnumType::Invalid,
                    cache_expiry_date_time: None,
                    charging_priority: None,
                    language1: None,
                    evse_id: None,
                    language2: None,
                    group_id_token: None,
                    personal_message: None,
                },
            }),
            IdTokenEnumType::ISO14443 => self.authorize_rfid_tag(&request.id_token.id_token).await,
            IdTokenEnumType::ISO15693 => self.authorize_rfid_tag(&request.id_token.id_token).await,
            IdTokenEnumType::KeyCode => Ok(AuthorizeResponse {
                certificate_status: None,
                id_token_info: IdTokenInfoType {
                    status: AuthorizationStatusEnumType::Invalid,
                    cache_expiry_date_time: None,
                    charging_priority: None,
                    language1: None,
                    evse_id: None,
                    language2: None,
                    group_id_token: None,
                    personal_message: None,
                },
            }),
            // This one is special, and will need some sort of callback to validate the id
            IdTokenEnumType::Local => Ok(AuthorizeResponse {
                certificate_status: None,
                id_token_info: IdTokenInfoType {
                    status: AuthorizationStatusEnumType::Invalid,
                    cache_expiry_date_time: None,
                    charging_priority: None,
                    language1: None,
                    evse_id: None,
                    language2: None,
                    group_id_token: None,
                    personal_message: None,
                },
            }),
            IdTokenEnumType::MacAddress => Err(OCPP2_0_1Error::new_internal_str(
                "Mac address authorization is not supported",
            )),
            IdTokenEnumType::NoAuthorization => Ok(AuthorizeResponse {
                certificate_status: None,
                id_token_info: IdTokenInfoType {
                    status: AuthorizationStatusEnumType::Accepted,
                    cache_expiry_date_time: None,
                    charging_priority: None,
                    language1: None,
                    evse_id: None,
                    language2: None,
                    group_id_token: None,
                    personal_message: None,
                },
            }),
        }
    }

    #[instrument(skip(self))]
    pub async fn handle_boot_notification(
        &mut self,
        request: BootNotificationRequest,
    ) -> Result<BootNotificationResponse, OCPP2_0_1Error> {
        self.charger.data.vendor = Some(request.charging_station.vendor_name.clone());
        self.charger.data.serial_number = request.charging_station.serial_number;
        self.charger.data.firmware_version = request.charging_station.firmware_version;
        self.charger.data.model = Some(request.charging_station.model);

        if let Some(modem) = &request.charging_station.modem {
            self.charger.data.iccid = modem.iccid.clone();
            self.charger.data.imsi = modem.imsi.clone();
        }

        if let Err(err) = self.charger.sync_data().await {
            error!(
                error_message = err.to_string(),
                "Failed to update charger database"
            );
        }

        if self.charger.authenticated {
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

    #[instrument(skip(self))]
    pub async fn handle_cleared_charging_limit(
        &mut self,
        _request: ClearedChargingLimitRequest,
    ) -> Result<ClearedChargingLimitResponse, OCPP2_0_1Error> {
        Ok(ClearedChargingLimitResponse {})
    }

    #[instrument(skip(self))]
    pub async fn handle_data_transfer(
        &mut self,
        _request: DataTransferRequest,
    ) -> Result<DataTransferResponse, OCPP2_0_1Error> {
        Ok(DataTransferResponse {
            status: DataTransferStatusEnumType::Rejected,
            data: None,
            status_info: None,
        })
    }

    #[instrument(skip(self))]
    pub async fn handle_firmware_status_notification(
        &mut self,
        _request: FirmwareStatusNotificationRequest,
    ) -> Result<FirmwareStatusNotificationResponse, OCPP2_0_1Error> {
        Ok(FirmwareStatusNotificationResponse {})
    }

    #[instrument(skip(self))]
    pub async fn handle_get_15118_ev_certificate(
        &mut self,
        _request: Get15118EVCertificateRequest,
    ) -> Result<Get15118EVCertificateResponse, OCPP2_0_1Error> {
        todo!()
    }

    #[instrument(skip(self))]
    pub async fn handle_get_certificate_status(
        &mut self,
        _request: GetCertificateStatusRequest,
    ) -> Result<GetCertificateStatusResponse, OCPP2_0_1Error> {
        todo!()
    }

    #[instrument(skip(self))]
    pub async fn handle_heartbeat(
        &mut self,
        _request: HeartbeatRequest,
    ) -> Result<HeartbeatResponse, OCPP2_0_1Error> {
        Ok(HeartbeatResponse {
            current_time: Utc::now(),
        })
    }

    #[instrument(skip(self))]
    pub async fn handle_log_status_notification(
        &mut self,
        _request: LogStatusNotificationRequest,
    ) -> Result<LogStatusNotificationResponse, OCPP2_0_1Error> {
        Ok(LogStatusNotificationResponse {})
    }

    #[instrument(skip(self))]
    pub async fn handle_meter_values(
        &mut self,
        _request: MeterValuesRequest,
    ) -> Result<MeterValuesResponse, OCPP2_0_1Error> {
        Ok(MeterValuesResponse {})
    }

    #[instrument(skip(self))]
    pub async fn handle_notify_customer_information(
        &mut self,
        _request: NotifyCustomerInformationRequest,
    ) -> Result<NotifyCustomerInformationResponse, OCPP2_0_1Error> {
        Ok(NotifyCustomerInformationResponse {})
    }

    #[instrument(skip(self))]
    pub async fn handle_notify_display_messages(
        &mut self,
        _request: NotifyDisplayMessagesRequest,
    ) -> Result<NotifyDisplayMessagesResponse, OCPP2_0_1Error> {
        Ok(NotifyDisplayMessagesResponse {})
    }

    #[instrument(skip(self))]
    pub async fn handle_notify_ev_charging_needs(
        &mut self,
        _request: NotifyEVChargingNeedsRequest,
    ) -> Result<NotifyEVChargingNeedsResponse, OCPP2_0_1Error> {
        todo!()
    }

    #[instrument(skip(self))]
    pub async fn handle_notify_ev_charging_schedule(
        &mut self,
        _request: NotifyEVChargingScheduleRequest,
    ) -> Result<NotifyEVChargingScheduleResponse, OCPP2_0_1Error> {
        todo!()
    }

    #[instrument(skip(self))]
    pub async fn handle_notify_event(
        &mut self,
        _request: NotifyEventRequest,
    ) -> Result<NotifyEventResponse, OCPP2_0_1Error> {
        Ok(NotifyEventResponse {})
    }

    #[instrument(skip(self))]
    pub async fn handle_notify_monitoring_report(
        &mut self,
        _request: NotifyMonitoringReportRequest,
    ) -> Result<NotifyMonitoringReportResponse, OCPP2_0_1Error> {
        Ok(NotifyMonitoringReportResponse {})
    }

    #[instrument(skip(self))]
    pub async fn handle_notify_report(
        &mut self,
        _request: NotifyReportRequest,
    ) -> Result<NotifyReportResponse, OCPP2_0_1Error> {
        Ok(NotifyReportResponse {})
    }

    #[instrument(skip(self))]
    pub async fn handle_publish_firmware_status_notification(
        &mut self,
        _request: PublishFirmwareStatusNotificationRequest,
    ) -> Result<PublishFirmwareStatusNotificationResponse, OCPP2_0_1Error> {
        Ok(PublishFirmwareStatusNotificationResponse {})
    }

    #[instrument(skip(self))]
    pub async fn handle_report_charging_profiles(
        &mut self,
        _request: ReportChargingProfilesRequest,
    ) -> Result<ReportChargingProfilesResponse, OCPP2_0_1Error> {
        Ok(ReportChargingProfilesResponse {})
    }

    #[instrument(skip(self))]
    pub async fn handle_request_start_transaction(
        &mut self,
        _request: RequestStartTransactionRequest,
    ) -> Result<RequestStartTransactionResponse, OCPP2_0_1Error> {
        todo!()
    }

    #[instrument(skip(self))]
    pub async fn handle_request_stop_transaction(
        &mut self,
        _request: RequestStopTransactionRequest,
    ) -> Result<RequestStopTransactionResponse, OCPP2_0_1Error> {
        todo!()
    }

    #[instrument(skip(self))]
    pub async fn handle_reservation_status_update(
        &mut self,
        _request: ReservationStatusUpdateRequest,
    ) -> Result<ReservationStatusUpdateResponse, OCPP2_0_1Error> {
        Ok(ReservationStatusUpdateResponse {})
    }

    #[instrument(skip(self))]
    pub async fn handle_security_event_notification(
        &mut self,
        _request: SecurityEventNotificationRequest,
    ) -> Result<SecurityEventNotificationResponse, OCPP2_0_1Error> {
        Ok(SecurityEventNotificationResponse {})
    }

    #[instrument(skip(self))]
    pub async fn handle_sign_certificate(
        &mut self,
        _request: SignCertificateRequest,
    ) -> Result<SignCertificateResponse, OCPP2_0_1Error> {
        todo!()
    }

    #[instrument(skip(self))]
    pub async fn handle_status_notification(
        &mut self,
        request: StatusNotificationRequest,
    ) -> Result<StatusNotificationResponse, OCPP2_0_1Error> {
        if let Some(evse) = self
            .charger
            .data
            .evse_by_ocpp_id_mut(request.evse_id as u32)
        {
            let evse_id = evse.id;
            if let Some(connector) = evse.connector_by_ocpp_id_mut(request.connector_id as u32) {
                connector.status = Some(request.connector_status.clone().into());

                self.charger
                    .event_manager
                    .send_connector_status_event(
                        self.charger.id.clone(),
                        request.connector_status.into(),
                        request.timestamp,
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
        Ok(StatusNotificationResponse {})
    }

    #[instrument(skip(self))]
    pub async fn handle_transaction_event(
        &mut self,
        _request: TransactionEventRequest,
    ) -> Result<TransactionEventResponse, OCPP2_0_1Error> {
        todo!()
    }
}
