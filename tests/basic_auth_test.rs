use crate::setup_new_client::setup_new_client;

mod setup_new_client;


#[tokio::test(flavor = "multi_thread")]
async fn it_should_connect_and_upgrade_the_password() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let client = setup_new_client().await?;
    client.disconnect().await?;
    Ok(())
}