use crate::setup_new_client::setup_new_ocpp1_6_client;
use chrono::Utc;
use rust_ocpp::v1_6::messages::status_notification::StatusNotificationRequest;

mod setup_new_client;

#[tokio::test(flavor = "multi_thread")]
async fn it_shoud_send_status_notification_successfully()
-> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let (client, _) = setup_new_ocpp1_6_client().await?;

    client
        .send_status_notification(StatusNotificationRequest {
            connector_id: 1,
            error_code: Default::default(),
            info: None,
            status: Default::default(),
            timestamp: Some(Utc::now()),
            vendor_id: None,
            vendor_error_code: None,
        })
        .await??;

    client.disconnect().await?;
    Ok(())
}
