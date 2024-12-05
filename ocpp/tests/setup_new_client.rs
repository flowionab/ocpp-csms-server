use futures::future::join3;
use ocpp_client::ocpp_1_6::OCPP1_6Client;
use ocpp_client::rust_ocpp::v1_6::messages::boot_notification::BootNotificationRequest;
use ocpp_client::{connect_1_6, ConnectOptions};
use rust_ocpp::v1_6::messages::change_configuration::ChangeConfigurationResponse;
use rust_ocpp::v1_6::messages::get_configuration::GetConfigurationResponse;
use rust_ocpp::v1_6::types::{ConfigurationStatus, RegistrationStatus};
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

pub async fn setup_new_client(
) -> Result<(OCPP1_6Client, String), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let username = Uuid::new_v4().to_string();
    let client = connect_1_6(&format!("ws://localhost:3001/{}", username), None).await?;

    let fut1 = client.send_boot_notification(BootNotificationRequest {
        charge_box_serial_number: Some("TEST123123".to_string()),
        charge_point_model: "TEST_MODEL".to_string(),
        charge_point_serial_number: Some("TEST123123".to_string()),
        charge_point_vendor: "TEST_VENDOR".to_string(),
        firmware_version: None,
        iccid: None,
        imsi: None,
        meter_serial_number: None,
        meter_type: None,
    });

    let fut2 = client.wait_for_get_configuration(|_request, _| async move {
        Ok(GetConfigurationResponse {
            configuration_key: Some(vec![]),
            unknown_key: None,
        })
    });

    let fut3 = client.wait_for_change_configuration(|_request, _| async move {
        Ok(ChangeConfigurationResponse {
            status: ConfigurationStatus::Accepted,
        })
    });

    let (boot_result, get_result, change_result) = join3(fut1, fut2, fut3).await;

    let boot_request = boot_result??;
    assert_eq!(boot_request.status, RegistrationStatus::Pending);
    get_result?;
    let change_request = change_result?;

    let password = String::from_utf8(hex::decode(&change_request.value)?)?;
    client.disconnect().await?;
    sleep(Duration::from_millis(2000)).await;
    let client = connect_1_6(
        &format!("ws://localhost:3001/{}", username),
        Some(ConnectOptions {
            username: Some(&username),
            password: Some(&password),
        }),
    )
    .await?;

    let response = client
        .send_boot_notification(BootNotificationRequest {
            charge_box_serial_number: Some("TEST123123".to_string()),
            charge_point_model: "TEST_MODEL".to_string(),
            charge_point_serial_number: Some("TEST123123".to_string()),
            charge_point_vendor: "TEST_VENDOR".to_string(),
            firmware_version: None,
            iccid: None,
            imsi: None,
            meter_serial_number: None,
            meter_type: None,
        })
        .await??;

    assert_eq!(response.status, RegistrationStatus::Accepted);

    Ok((client, username.to_string()))
}
