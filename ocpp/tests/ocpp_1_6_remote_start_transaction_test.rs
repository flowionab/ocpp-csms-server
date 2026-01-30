use crate::ocpp_csms_server::StartTransactionRequest;
use crate::ocpp_csms_server::ocpp_client::OcppClient;
use crate::setup_new_client::setup_new_ocpp1_6_client;
use rust_ocpp::v1_6::messages::remote_start_transaction::RemoteStartTransactionResponse;
use rust_ocpp::v1_6::types::RemoteStartStopStatus;
use tokio::try_join;

mod setup_new_client;

pub mod ocpp_csms_server {
    tonic::include_proto!("ocpp_csms_server");
}

#[tokio::test(flavor = "multi_thread")]
async fn calling_remote_start_transaction_should_work()
-> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let (client, username) = setup_new_ocpp1_6_client().await?;
    let mut grpc_client = OcppClient::connect("http://[::1]:50051").await?;

    try_join!(
        client.wait_for_remote_start_transaction(|_request, _| async move {
            Ok(RemoteStartTransactionResponse {
                status: RemoteStartStopStatus::Accepted,
            })
        }),
        async {
            grpc_client
                .start_transaction(StartTransactionRequest {
                    charger_id: username.to_string(),
                    outlet_id: "".to_string(),
                })
                .await
                .map_err(|e| e.into())
        }
    )?;

    client.disconnect().await?;
    Ok(())
}
