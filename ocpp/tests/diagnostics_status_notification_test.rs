use rust_ocpp::v1_6::messages::diagnostics_status_notification::DiagnosticsStatusNotificationRequest;
use rust_ocpp::v1_6::types::{DiagnosticsStatus};
use crate::setup_new_client::setup_new_client;

mod setup_new_client;

#[tokio::test(flavor = "multi_thread")]
async fn it_should_handle_diagnostics_status_notification() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let client = setup_new_client().await?;
    
    client.send_diagnostics_status_notification(DiagnosticsStatusNotificationRequest { status: DiagnosticsStatus::Uploading }).await??;

    client.disconnect().await?;
    Ok(())
}