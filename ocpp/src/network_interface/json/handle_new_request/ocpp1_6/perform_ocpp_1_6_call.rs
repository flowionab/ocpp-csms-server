use crate::network_interface::json::handle_new_request::ocpp1_6::handle_ocpp_1_6_request::handle_ocpp_1_6_request;
use crate::network_interface::ocpp1_6_request_receiver::Ocpp16RequestReceiver;
use ocpp_client::ocpp_1_6::OCPP1_6Error;
use serde_json::Value;
use std::time::Duration;

pub async fn perform_ocpp_1_6_call<T: Ocpp16RequestReceiver + Send + Sync + 'static>(
    duration: Duration,
    charger: &mut T,
    action: &str,
    payload: Value,
) -> Result<Result<Value, OCPP1_6Error>, Box<dyn std::error::Error + Send + Sync>> {
    let result = match action {
        "Authorize" => {
            handle_ocpp_1_6_request(
                duration,
                charger.authorize(serde_json::from_value(payload)?),
            )
            .await
        }
        "BootNotification" => {
            handle_ocpp_1_6_request(
                duration,
                charger.boot_notification(serde_json::from_value(payload)?),
            )
            .await
        }
        "DataTransfer" => {
            handle_ocpp_1_6_request(
                duration,
                charger.data_transfer(serde_json::from_value(payload)?),
            )
            .await
        }
        "DiagnosticsStatusNotification" => {
            handle_ocpp_1_6_request(
                duration,
                charger.diagnostics_status_notification(serde_json::from_value(payload)?),
            )
            .await
        }
        "FirmwareStatusNotification" => {
            handle_ocpp_1_6_request(
                duration,
                charger.firmware_status_notification(serde_json::from_value(payload)?),
            )
            .await
        }
        "Heartbeat" => {
            handle_ocpp_1_6_request(
                duration,
                charger.heartbeat(serde_json::from_value(payload)?),
            )
            .await
        }
        "MeterValues" => {
            handle_ocpp_1_6_request(
                duration,
                charger.meter_values(serde_json::from_value(payload)?),
            )
            .await
        }
        "StartTransaction" => {
            handle_ocpp_1_6_request(
                duration,
                charger.start_transaction(serde_json::from_value(payload)?),
            )
            .await
        }
        "StatusNotification" => {
            handle_ocpp_1_6_request(
                duration,
                charger.status_notification(serde_json::from_value(payload)?),
            )
            .await
        }
        "StopTransaction" => {
            handle_ocpp_1_6_request(
                duration,
                charger.stop_transaction(serde_json::from_value(payload)?),
            )
            .await
        }
        _ => Err(OCPP1_6Error::new_not_implemented(&format!(
            "Action '{}' is not implemented on this server",
            action
        ))),
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network_interface::ocpp1_6_request_receiver::MockOcpp16RequestReceiver;
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

    #[tokio::test]
    async fn test_authorize() {
        let mut mock = MockOcpp16RequestReceiver::new();
        mock.expect_authorize()
            .return_once(|_| Ok(AuthorizeResponse::default()));
        perform_ocpp_1_6_call(
            Duration::from_secs(1),
            &mut mock,
            "Authorize",
            serde_json::to_value(AuthorizeRequest::default()).unwrap(),
        )
        .await
        .unwrap()
        .unwrap();
        mock.checkpoint();
    }

    #[tokio::test]
    async fn test_boot_notification() {
        let mut mock = MockOcpp16RequestReceiver::new();
        mock.expect_boot_notification()
            .return_once(|_| Ok(BootNotificationResponse::default()));
        perform_ocpp_1_6_call(
            Duration::from_secs(1),
            &mut mock,
            "BootNotification",
            serde_json::to_value(BootNotificationRequest::default()).unwrap(),
        )
        .await
        .unwrap()
        .unwrap();
        mock.checkpoint();
    }

    #[tokio::test]
    async fn test_data_transfer() {
        let mut mock = MockOcpp16RequestReceiver::new();
        mock.expect_data_transfer()
            .return_once(|_| Ok(DataTransferResponse::default()));
        perform_ocpp_1_6_call(
            Duration::from_secs(1),
            &mut mock,
            "DataTransfer",
            serde_json::to_value(DataTransferRequest::default()).unwrap(),
        )
        .await
        .unwrap()
        .unwrap();
        mock.checkpoint();
    }

    #[tokio::test]
    async fn test_diagnostics_status_notification() {
        let mut mock = MockOcpp16RequestReceiver::new();
        mock.expect_diagnostics_status_notification()
            .return_once(|_| Ok(DiagnosticsStatusNotificationResponse::default()));
        perform_ocpp_1_6_call(
            Duration::from_secs(1),
            &mut mock,
            "DiagnosticsStatusNotification",
            serde_json::to_value(DiagnosticsStatusNotificationRequest::default()).unwrap(),
        )
        .await
        .unwrap()
        .unwrap();
        mock.checkpoint();
    }

    #[tokio::test]
    async fn test_firmware_status_notification() {
        let mut mock = MockOcpp16RequestReceiver::new();
        mock.expect_firmware_status_notification()
            .return_once(|_| Ok(FirmwareStatusNotificationResponse::default()));
        perform_ocpp_1_6_call(
            Duration::from_secs(1),
            &mut mock,
            "FirmwareStatusNotification",
            serde_json::to_value(FirmwareStatusNotificationRequest::default()).unwrap(),
        )
        .await
        .unwrap()
        .unwrap();
        mock.checkpoint();
    }

    #[tokio::test]
    async fn test_heartbeat() {
        let mut mock = MockOcpp16RequestReceiver::new();
        mock.expect_heartbeat()
            .return_once(|_| Ok(HeartbeatResponse::default()));
        perform_ocpp_1_6_call(
            Duration::from_secs(1),
            &mut mock,
            "Heartbeat",
            serde_json::to_value(HeartbeatRequest::default()).unwrap(),
        )
        .await
        .unwrap()
        .unwrap();
        mock.checkpoint();
    }

    #[tokio::test]
    async fn test_meter_values() {
        let mut mock = MockOcpp16RequestReceiver::new();
        mock.expect_meter_values()
            .return_once(|_| Ok(MeterValuesResponse::default()));
        perform_ocpp_1_6_call(
            Duration::from_secs(1),
            &mut mock,
            "MeterValues",
            serde_json::to_value(MeterValuesRequest::default()).unwrap(),
        )
        .await
        .unwrap()
        .unwrap();
        mock.checkpoint();
    }

    #[tokio::test]
    async fn test_start_transaction() {
        let mut mock = MockOcpp16RequestReceiver::new();
        mock.expect_start_transaction()
            .return_once(|_| Ok(StartTransactionResponse::default()));
        perform_ocpp_1_6_call(
            Duration::from_secs(1),
            &mut mock,
            "StartTransaction",
            serde_json::to_value(StartTransactionRequest::default()).unwrap(),
        )
        .await
        .unwrap()
        .unwrap();
        mock.checkpoint();
    }

    #[tokio::test]
    async fn test_status_notification() {
        let mut mock = MockOcpp16RequestReceiver::new();
        mock.expect_status_notification()
            .return_once(|_| Ok(StatusNotificationResponse::default()));
        perform_ocpp_1_6_call(
            Duration::from_secs(1),
            &mut mock,
            "StatusNotification",
            serde_json::to_value(StatusNotificationRequest::default()).unwrap(),
        )
        .await
        .unwrap()
        .unwrap();
        mock.checkpoint();
    }

    #[tokio::test]
    async fn test_stop_transaction() {
        let mut mock = MockOcpp16RequestReceiver::new();
        mock.expect_stop_transaction()
            .return_once(|_| Ok(StopTransactionResponse::default()));
        perform_ocpp_1_6_call(
            Duration::from_secs(1),
            &mut mock,
            "StopTransaction",
            serde_json::to_value(StopTransactionRequest::default()).unwrap(),
        )
        .await
        .unwrap()
        .unwrap();
        mock.checkpoint();
    }

    #[tokio::test]
    async fn it_should_return_err_on_invalid_action() {
        let mut mock = MockOcpp16RequestReceiver::new();
        let err = perform_ocpp_1_6_call(
            Duration::from_secs(1),
            &mut mock,
            "InvalidAction",
            serde_json::to_value(StopTransactionRequest::default()).unwrap(),
        )
        .await
        .unwrap()
        .unwrap_err();
        mock.checkpoint();

        assert_eq!(err.code(), "NotImplemented");
    }
}
