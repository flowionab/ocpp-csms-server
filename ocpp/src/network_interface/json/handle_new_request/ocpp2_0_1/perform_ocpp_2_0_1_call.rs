use crate::network_interface::json::handle_new_request::ocpp2_0_1::handle_ocpp_2_0_1_request::handle_ocpp_2_0_1_request;
use crate::network_interface::ocpp2_0_1_request_receiver::Ocpp2_0_1RequestReceiver;
use ocpp_client::ocpp_2_0_1::OCPP2_0_1Error;
use serde_json::Value;
use std::time::Duration;

pub async fn perform_ocpp_2_0_1_call<T: Ocpp2_0_1RequestReceiver + Send + Sync + 'static>(
    duration: Duration,
    charger: &mut T,
    action: &str,
    payload: Value,
) -> Result<Result<Value, OCPP2_0_1Error>, Box<dyn std::error::Error + Send + Sync>> {
    let result = match action {
        "Authorize" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.authorize(serde_json::from_value(payload)?),
            )
            .await
        }
        "BootNotification" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.boot_notification(serde_json::from_value(payload)?),
            )
            .await
        }
        "ClearedChargingLimit" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.cleared_charging_limit(serde_json::from_value(payload)?),
            )
            .await
        }
        "DataTransfer" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.data_transfer(serde_json::from_value(payload)?),
            )
            .await
        }
        "FirmwareStatusNotification" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.firmware_status_notification(serde_json::from_value(payload)?),
            )
            .await
        }
        "Get15118EVCertificate" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.get_15118_ev_certificate(serde_json::from_value(payload)?),
            )
            .await
        }
        "GetCertificateStatus" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.get_certificate_status(serde_json::from_value(payload)?),
            )
            .await
        }
        "Heartbeat" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.heartbeat(serde_json::from_value(payload)?),
            )
            .await
        }
        "LogStatusNotification" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.log_status_notification(serde_json::from_value(payload)?),
            )
            .await
        }
        "MeterValues" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.meter_values(serde_json::from_value(payload)?),
            )
            .await
        }
        "NotifyCustomerInformation" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.notify_customer_information(serde_json::from_value(payload)?),
            )
            .await
        }
        "NotifyDisplayMessages" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.notify_display_messages(serde_json::from_value(payload)?),
            )
            .await
        }
        "NotifyEVChargingNeeds" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.notify_ev_charging_needs(serde_json::from_value(payload)?),
            )
            .await
        }
        "NotifyEVChargingSchedule" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.notify_ev_charging_schedule(serde_json::from_value(payload)?),
            )
            .await
        }
        "NotifyEvent" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.notify_event(serde_json::from_value(payload)?),
            )
            .await
        }
        "NotifyMonitoringReport" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.notify_monitoring_report(serde_json::from_value(payload)?),
            )
            .await
        }
        "NotifyReport" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.notify_report(serde_json::from_value(payload)?),
            )
            .await
        }
        "PublishFirmwareStatusNotification" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.publish_firmware_status_notification(serde_json::from_value(payload)?),
            )
            .await
        }
        "ReportChargingProfiles" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.report_charging_profiles(serde_json::from_value(payload)?),
            )
            .await
        }
        "RequestStartTransaction" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.request_start_transaction(serde_json::from_value(payload)?),
            )
            .await
        }
        "RequestStopTransaction" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.request_stop_transaction(serde_json::from_value(payload)?),
            )
            .await
        }
        "ReservationStatusUpdate" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.reservation_status_update(serde_json::from_value(payload)?),
            )
            .await
        }
        "SecurityEventNotification" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.security_event_notification(serde_json::from_value(payload)?),
            )
            .await
        }
        "SignCertificate" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.sign_certificate(serde_json::from_value(payload)?),
            )
            .await
        }
        "StatusNotification" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.status_notification(serde_json::from_value(payload)?),
            )
            .await
        }
        "TransactionEvent" => {
            handle_ocpp_2_0_1_request(
                duration,
                charger.transaction_event(serde_json::from_value(payload)?),
            )
            .await
        }
        _ => Err(OCPP2_0_1Error::new_not_implemented(&format!(
            "Action '{}' is not implemented on this server",
            action
        ))),
    };
    Ok(result)
}
