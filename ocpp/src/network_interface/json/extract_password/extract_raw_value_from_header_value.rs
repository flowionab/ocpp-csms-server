use poem::http::{HeaderValue, StatusCode};
use poem::Response;
use tracing::log::warn;

#[allow(clippy::result_large_err)]
pub fn extract_raw_value_from_header_value(header: &HeaderValue) -> Result<String, Response> {
    let value = header.to_str().map_err(|_e| {
        warn!("Authorization header was provided, but the content was not a valid string");
        Response::builder().status(StatusCode::BAD_REQUEST).body(
            "Authorization header was provided, but the content was not a valid string".to_string(),
        )
    })?;
    Ok(value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::http::HeaderValue;
    use poem::http::StatusCode;

    #[test]
    fn test_extract_valid_header_value() {
        let header = HeaderValue::from_static("Basic dXNlcjpwYXNz");
        let result = extract_raw_value_from_header_value(&header);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Basic dXNlcjpwYXNz");
    }

    #[test]
    fn test_extract_invalid_header_value() {
        // HeaderValue::from_bytes with invalid bytes (e.g., 0xFF) will fail at creation,
        // so we use unsafe to create an invalid HeaderValue for testing.
        let invalid_bytes = b"\xFF\xFF\xFF";
        let header = HeaderValue::from_bytes(invalid_bytes).unwrap();
        let result = extract_raw_value_from_header_value(&header);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
        let body = err.into_body().into_string();
        // `into_string` is async, so we need to block on it
        let body = futures::executor::block_on(body).unwrap();
        assert!(body.contains("not a valid string"));
    }
}
