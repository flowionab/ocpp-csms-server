use crate::setup_new_client::setup_new_ocpp1_6_client;
use chrono::Utc;
use rust_ocpp::v1_6::messages::start_transaction::StartTransactionRequest;
use rust_ocpp::v1_6::types::AuthorizationStatus;

mod setup_new_client;

#[tokio::test(flavor = "multi_thread")]
async fn it_should_handle_start_transaction()
-> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let (client, _) = setup_new_ocpp1_6_client().await?;

    let result = client
        .send_start_transaction(StartTransactionRequest {
            connector_id: 1,
            id_tag: "test123".to_string(),
            meter_start: 0,
            reservation_id: None,
            timestamp: Utc::now(),
        })
        .await??;

    assert_eq!(result.id_tag_info.status, AuthorizationStatus::Invalid);

    client.disconnect().await?;
    Ok(())
}
