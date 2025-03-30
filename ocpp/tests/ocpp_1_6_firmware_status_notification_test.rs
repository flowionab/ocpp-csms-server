use crate::setup_new_client::setup_new_ocpp1_6_client;
use rust_ocpp::v1_6::messages::firmware_status_notification::FirmwareStatusNotificationRequest;
use rust_ocpp::v1_6::types::FirmwareStatus;

mod setup_new_client;

#[tokio::test(flavor = "multi_thread")]
async fn it_should_handle_firmware_status_notification(
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let (client, _) = setup_new_ocpp1_6_client().await?;

    client
        .send_firmware_status_notification(FirmwareStatusNotificationRequest {
            status: FirmwareStatus::Downloaded,
        })
        .await??;

    client.disconnect().await?;
    Ok(())
}
