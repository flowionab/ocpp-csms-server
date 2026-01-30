use poem::Response;
use poem::http::StatusCode;
use tracing::log::warn;

#[allow(clippy::result_large_err)]
pub fn get_encoded_part(value: &str) -> Result<String, Response> {
    let splits = value.split(" ").collect::<Vec<_>>();
    match splits.first() {
        None => {
            warn!(
                "Authorization header was provided, but the content should start with Basic and have base64 encoded credentials"
            );
            return Err(Response::builder().status(StatusCode::BAD_REQUEST).body(
                "Authorization header was provided, but the content should start with Basic and have base64 encoded credentials".to_string(),
            ));
        }
        Some(val) => {
            if *val != "Basic" {
                warn!(
                    "Authorization header was provided, but the only supported credential type is Basic"
                );
                return Err(Response::builder().status(StatusCode::BAD_REQUEST).body(
                    "Authorization header was provided, but the only supported credential type is Basic".to_string(),
                ));
            }
        }
    }

    match splits.get(1) {
        None => {
            warn!("Authorization header was provided, but the base64 part was missing");
            Err(Response::builder().status(StatusCode::BAD_REQUEST).body(
                "Authorization header was provided, but the base64 part was missing".to_string(),
            ))
        }
        Some(val) => Ok(val.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::http::StatusCode;

    #[test]
    fn test_valid_basic_with_base64() {
        let input = "Basic dXNlcjpwYXNz";
        let result = get_encoded_part(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "dXNlcjpwYXNz");
    }

    #[tokio::test]
    async fn test_missing_type() {
        let input = "";
        let result = get_encoded_part(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
        let body = err.into_body().into_string().await.unwrap();
        println!("Body: {}", body);
        assert!(body.contains("only supported credential type is Basic"));
    }

    #[tokio::test]
    async fn test_wrong_type() {
        let input = "Bearer sometoken";
        let result = get_encoded_part(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
        let body = err.into_body().into_string().await.unwrap();
        assert!(body.contains("only supported credential type is Basic"));
    }

    #[tokio::test]
    async fn test_missing_base64_part() {
        let input = "Basic";
        let result = get_encoded_part(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
        let body = err.into_body().into_string().await.unwrap();
        assert!(body.contains("base64 part was missing"));
    }
}
