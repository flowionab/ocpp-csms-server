use std::time::Duration;
use ocpp_client::{connect_1_6, ConnectOptions};
use ocpp_client::rust_ocpp::v1_6::messages::boot_notification::BootNotificationRequest;
use rust_ocpp::v1_6::messages::change_configuration::ChangeConfigurationResponse;
use rust_ocpp::v1_6::messages::get_configuration::GetConfigurationResponse;
use rust_ocpp::v1_6::types::ConfigurationStatus;
use tokio::time::sleep;

#[tokio::test]
async fn it_should_connect() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let client = connect_1_6("ws://localhost:3000/TEST123123", Some(ConnectOptions {
        username: Some("TEST123123"),
        password: Some("123123")
    })).await?;

    client.on_get_configuration(|_request, _| async move {
        Ok(GetConfigurationResponse { configuration_key: Some(vec![]), unknown_key: None })
    }).await;

    client.on_change_configuration(|_request, _| async move {
        Ok(ChangeConfigurationResponse { status: ConfigurationStatus::Accepted })
    }).await;

    client.send_boot_notification(BootNotificationRequest {
        charge_box_serial_number: Some("TEST123123".to_string()),
        charge_point_model: "TEST_MODEL".to_string(),
        charge_point_serial_number: Some("TEST123123".to_string()),
        charge_point_vendor: "TEST_VENDOR".to_string(),
        firmware_version: None,
        iccid: None,
        imsi: None,
        meter_serial_number: None,
        meter_type: None,
    }).await??;

    sleep(Duration::from_millis(5000)).await;

    client.disconnect().await?;
    Ok(())
}