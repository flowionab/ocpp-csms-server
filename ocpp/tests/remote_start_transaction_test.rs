use rust_ocpp::v1_6::messages::remote_start_transaction::RemoteStartTransactionResponse;
use rust_ocpp::v1_6::types::RemoteStartStopStatus;
use crate::setup_new_client::setup_new_client;

mod setup_new_client;

#[tokio::test(flavor = "multi_thread")]
async fn calling_remote_start_transaction_should_work() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let client = setup_new_client().await?;

    client.wait_for_remote_start_transaction(|_request| async move {
        RemoteStartTransactionResponse {
            status: RemoteStartStopStatus::Accepted,
        }
    }).await?;

    client.disconnect().await?;
    Ok(())
}