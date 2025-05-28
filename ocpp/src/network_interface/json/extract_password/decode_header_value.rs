use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use poem::http::StatusCode;
use poem::Response;
use std::str::from_utf8;
use tracing::log::warn;

#[allow(clippy::result_large_err)]
pub fn decode_header_value(value: &str) -> Result<String, Response> {
    let bytes = BASE64_STANDARD.decode(value).map_err(|_e| {
        warn!(
            "Authorization header was provided, but the content was not correctly base64 encoded"
        );
        Response::builder().status(StatusCode::BAD_REQUEST).body(
            "Authorization header was provided, but the content was not correctly base64 encoded"
                .to_string(),
        )
    })?;

    let string = from_utf8(&bytes).map_err(|_e| {
        warn!("Authorization header was provided, but the content was not correct UTF-8");
        Response::builder().status(StatusCode::BAD_REQUEST).body(
            "Authorization header was provided, but the content was not correct UTF-8".to_string(),
        )
    })?;

    Ok(string.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::http::StatusCode;

    #[test]
    fn test_decode_valid_base64() {
        // "user:pass" base64-encoded is "dXNlcjpwYXNz"
        let input = "dXNlcjpwYXNz";
        let result = decode_header_value(input);
        assert_eq!(result.unwrap(), "user:pass");
    }

    #[tokio::test]
    async fn test_decode_invalid_base64() {
        let input = "!!!not_base64!!!";
        let result = decode_header_value(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
        let body = err.into_body().into_string().await.unwrap();
        assert!(body.contains("not correctly base64 encoded"));
    }

    #[tokio::test]
    async fn test_decode_invalid_utf8() {
        // 0xFF is not valid UTF-8
        let input = base64::engine::general_purpose::STANDARD.encode([0xFF, 0xFF, 0xFF]);
        let result = decode_header_value(&input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
        let body = err.into_body().into_string().await.unwrap();
        assert!(body.contains("not correct UTF-8"));
    }
}
