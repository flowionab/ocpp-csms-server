use crate::setup_new_client::setup_new_client;
use rust_ocpp::v1_6::messages::data_transfer::DataTransferRequest;
use rust_ocpp::v1_6::types::DataTransferStatus;

mod setup_new_client;

#[tokio::test(flavor = "multi_thread")]
async fn it_should_handle_data_transfer(
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let (client, _) = setup_new_client().await?;

    let result = client
        .send_data_transfer(DataTransferRequest {
            vendor_string: "".to_string(),
            message_id: None,
            data: None,
        })
        .await??;

    assert_eq!(result.status, DataTransferStatus::Rejected);

    client.disconnect().await?;
    Ok(())
}
