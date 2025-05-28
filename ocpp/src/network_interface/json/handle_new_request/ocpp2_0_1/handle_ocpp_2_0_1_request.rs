use ocpp_client::ocpp_2_0_1::OCPP2_0_1Error;
use serde::Serialize;
use serde_json::Value;
use std::future::Future;
use tokio::time::timeout;

pub async fn handle_ocpp_2_0_1_request<T: Serialize>(
    duration: core::time::Duration,
    future: impl Future<Output = Result<T, OCPP2_0_1Error>>,
) -> Result<Value, OCPP2_0_1Error> {
    timeout(duration, future)
        .await
        .map_err(|_| OCPP2_0_1Error::InternalError {
            description: format!("Request timed out after {}ms", duration.as_millis()),
            details: Value::Null,
        })?
        .map(|i| serde_json::to_value(&i).unwrap())
}
