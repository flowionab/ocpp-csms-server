use ocpp_client::ocpp_1_6::OCPP1_6Error;
use serde::Serialize;
use serde_json::Value;
use std::future::Future;
use tokio::time::timeout;

pub async fn handle_ocpp_1_6_request<T: Serialize>(
    duration: core::time::Duration,
    future: impl Future<Output = Result<T, OCPP1_6Error>>,
) -> Result<Value, OCPP1_6Error> {
    timeout(duration, future)
        .await
        .map_err(|_| OCPP1_6Error::InternalError {
            description: format!("Request timed out after {}ms", duration.as_millis()),
            details: Value::Null,
        })?
        .map(|i| serde_json::to_value(&i).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::FutureExt;
    use ocpp_client::ocpp_1_6::OCPP1_6Error;
    use serde::Serialize;
    use serde_json::json;
    use std::time::Duration;
    use tokio::time::sleep;

    #[derive(Serialize)]
    struct TestData {
        value: i32,
    }

    #[tokio::test]
    async fn test_successful_response() {
        let data = TestData { value: 42 };
        let fut = async { Ok(data) };
        let result = handle_ocpp_1_6_request(Duration::from_millis(100), fut).await;
        assert_eq!(result.unwrap(), json!({"value": 42}));
    }

    #[tokio::test]
    async fn test_timeout_error() {
        let fut = async {
            sleep(Duration::from_millis(200)).await;
            Ok(TestData { value: 1 })
        };
        let result = handle_ocpp_1_6_request(Duration::from_millis(50), fut).await;
        match result {
            Err(OCPP1_6Error::InternalError {
                description,
                details,
            }) => {
                assert!(description.contains("timed out"));
                assert_eq!(details, serde_json::Value::Null);
            }
            _ => panic!("Expected InternalError"),
        }
    }

    #[tokio::test]
    async fn test_serialization_error() {
        struct NonSerializable;
        impl Serialize for NonSerializable {
            fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                Err(serde::ser::Error::custom("fail"))
            }
        }
        let fut = async { Ok(NonSerializable) };
        // This will panic due to unwrap in map, so catch the panic
        let result =
            std::panic::AssertUnwindSafe(handle_ocpp_1_6_request(Duration::from_millis(100), fut))
                .catch_unwind()
                .await;
        assert!(result.is_err());
    }
}
