use ocpp_client::connect_1_6;
use ocpp_client::rust_ocpp::v1_6::messages::boot_notification::BootNotificationRequest;

#[tokio::test]
async fn it_should_connect() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let client = connect_1_6("ws://localhost:3000/TEST123123").await?;

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
    client.disconnect().await?;
    Ok(())
}