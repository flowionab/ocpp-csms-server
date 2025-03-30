use crate::setup_new_client::setup_new_ocpp1_6_client;
use rust_ocpp::v1_6::messages::heart_beat::HeartbeatRequest;

mod setup_new_client;

#[tokio::test(flavor = "multi_thread")]
async fn it_should_handle_heart_beat(
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let (client, _) = setup_new_ocpp1_6_client().await?;

    client.send_heartbeat(HeartbeatRequest {}).await??;

    client.disconnect().await?;
    Ok(())
}
