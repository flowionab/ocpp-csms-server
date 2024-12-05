use crate::setup_new_client::setup_new_client;
use rust_ocpp::v1_6::messages::meter_values::MeterValuesRequest;

mod setup_new_client;

#[tokio::test(flavor = "multi_thread")]
async fn it_should_handle_start_transaction(
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let (client, _) = setup_new_client().await?;

    client
        .send_meter_values(MeterValuesRequest {
            connector_id: 1,
            transaction_id: None,
            meter_value: vec![],
        })
        .await??;

    client.disconnect().await?;
    Ok(())
}
