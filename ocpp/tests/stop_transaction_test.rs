use crate::setup_new_client::setup_new_client;
use chrono::Utc;
use rust_ocpp::v1_6::messages::stop_transaction::StopTransactionRequest;

mod setup_new_client;

#[tokio::test(flavor = "multi_thread")]
async fn it_should_handle_stop_transaction(
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let (client, _) = setup_new_client().await?;

    client
        .send_stop_transaction(StopTransactionRequest {
            id_tag: None,
            meter_stop: 0,
            timestamp: Utc::now(),
            transaction_id: 0,
            reason: None,
            transaction_data: None,
        })
        .await??;

    client.disconnect().await?;
    Ok(())
}
