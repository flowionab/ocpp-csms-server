use futures::future::join3;
use ocpp_client::ocpp_1_6::OCPP1_6Client;
use ocpp_client::ocpp_2_0_1::OCPP2_0_1Client;
use ocpp_client::{connect_1_6, connect_2_0_1, ConnectOptions};
use rust_ocpp::v2_0_1::datatypes::charging_station_type::ChargingStationType;
use rust_ocpp::v2_0_1::enumerations::boot_reason_enum_type::BootReasonEnumType;
use rust_ocpp::v2_0_1::messages::get_base_report::GetBaseReportResponse;
use rust_ocpp::v2_0_1::messages::set_variables::SetVariablesResponse;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

pub async fn setup_new_ocpp1_6_client(
) -> Result<(OCPP1_6Client, String), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let username = Uuid::new_v4().to_string();
    let client = connect_1_6(&format!("ws://localhost:3001/{}", username), None).await?;

    let fut1 = client.send_boot_notification(
        rust_ocpp::v1_6::messages::boot_notification::BootNotificationRequest {
            charge_box_serial_number: Some("TEST123123".to_string()),
            charge_point_model: "TEST_MODEL".to_string(),
            charge_point_serial_number: Some("TEST123123".to_string()),
            charge_point_vendor: "TEST_VENDOR".to_string(),
            firmware_version: None,
            iccid: None,
            imsi: None,
            meter_serial_number: None,
            meter_type: None,
        },
    );

    let fut2 = client.wait_for_get_configuration(|_request, _| async move {
        Ok(
            rust_ocpp::v1_6::messages::get_configuration::GetConfigurationResponse {
                configuration_key: Some(vec![]),
                unknown_key: None,
            },
        )
    });

    let fut3 = client.wait_for_change_configuration(|_request, _| async move {
        Ok(
            rust_ocpp::v1_6::messages::change_configuration::ChangeConfigurationResponse {
                status: rust_ocpp::v1_6::types::ConfigurationStatus::Accepted,
            },
        )
    });

    let (boot_result, get_result, change_result) = join3(fut1, fut2, fut3).await;

    let boot_request = boot_result??;
    assert_eq!(
        boot_request.status,
        rust_ocpp::v1_6::types::RegistrationStatus::Pending
    );
    get_result?;
    let change_request = change_result?;

    let password = String::from_utf8(hex::decode(&change_request.value)?)?;
    client.disconnect().await?;
    sleep(Duration::from_millis(1000)).await;
    let client = connect_1_6(
        &format!("ws://localhost:3001/{}", username),
        Some(ConnectOptions {
            username: Some(&username),
            password: Some(&password),
        }),
    )
    .await?;

    let response = client
        .send_boot_notification(
            rust_ocpp::v1_6::messages::boot_notification::BootNotificationRequest {
                charge_box_serial_number: Some("TEST123123".to_string()),
                charge_point_model: "TEST_MODEL".to_string(),
                charge_point_serial_number: Some("TEST123123".to_string()),
                charge_point_vendor: "TEST_VENDOR".to_string(),
                firmware_version: None,
                iccid: None,
                imsi: None,
                meter_serial_number: None,
                meter_type: None,
            },
        )
        .await??;

    assert_eq!(
        response.status,
        rust_ocpp::v1_6::types::RegistrationStatus::Accepted
    );

    Ok((client, username.to_string()))
}

#[allow(dead_code)]
pub async fn setup_new_ocpp2_0_1_client(
) -> Result<(OCPP2_0_1Client, String), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let username = Uuid::new_v4().to_string();
    let client = connect_2_0_1(&format!("ws://localhost:3001/{}", username), None).await?;

    let fut1 = client.send_boot_notification(
        rust_ocpp::v2_0_1::messages::boot_notification::BootNotificationRequest {
            reason: BootReasonEnumType::PowerUp,
            charging_station: ChargingStationType {
                serial_number: Some("TEST123123".to_string()),
                model: "TEST_MODEL".to_string(),
                vendor_name: "TEST_VENDOR".to_string(),
                firmware_version: None,
                modem: None,
            },
        },
    );

    let fut2 = client.wait_for_get_base_report(|_request, _| async move {
        Ok(GetBaseReportResponse {
            status: rust_ocpp::v2_0_1::enumerations::generic_device_model_status_enum_type::GenericDeviceModelStatusEnumType::Accepted,
            status_info: None,
        })
    });

    let fut3 = client.wait_for_set_variables(|_request, _| async move {
        Ok(SetVariablesResponse {
            set_variable_result: vec![],
        })
    });

    let (boot_result, get_result, change_result) = join3(fut1, fut2, fut3).await;

    let boot_request = boot_result??;
    assert_eq!(boot_request.status, rust_ocpp::v2_0_1::enumerations::registration_status_enum_type::RegistrationStatusEnumType::Pending);
    get_result?;
    let change_request = change_result?;

    let password = String::from_utf8(hex::decode(
        &change_request
            .set_variable_data
            .get(0)
            .unwrap()
            .attribute_value,
    )?)?;
    client.disconnect().await?;
    sleep(Duration::from_millis(1000)).await;
    let client = connect_2_0_1(
        &format!("ws://localhost:3001/{}", username),
        Some(ConnectOptions {
            username: Some(&username),
            password: Some(&password),
        }),
    )
    .await?;

    let response = client
        .send_boot_notification(
            rust_ocpp::v2_0_1::messages::boot_notification::BootNotificationRequest {
                reason: BootReasonEnumType::PowerUp,
                charging_station: ChargingStationType {
                    serial_number: Some("TEST123123".to_string()),
                    model: "TEST_MODEL".to_string(),
                    vendor_name: "TEST_VENDOR".to_string(),
                    firmware_version: None,
                    modem: None,
                },
            },
        )
        .await??;

    assert_eq!(response.status, rust_ocpp::v2_0_1::enumerations::registration_status_enum_type::RegistrationStatusEnumType::Accepted);

    Ok((client, username.to_string()))
}
