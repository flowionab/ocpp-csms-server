use rust_ocpp::v1_6::messages::authorize::AuthorizeRequest;
use rust_ocpp::v1_6::types::AuthorizationStatus;
use crate::setup_new_client::setup_new_client;

mod setup_new_client;

#[tokio::test(flavor = "multi_thread")]
async fn it_should_handle_authorize() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let client = setup_new_client().await?;
    
    let result = client.send_authorize(AuthorizeRequest { id_tag: "test_123".to_string() }).await??;

    assert_eq!(result.id_tag_info.status, AuthorizationStatus::Invalid);

    client.disconnect().await?;
    Ok(())
}