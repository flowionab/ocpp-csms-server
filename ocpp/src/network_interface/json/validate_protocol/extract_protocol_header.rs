use poem::{
    http::{HeaderMap, StatusCode},
    Response,
};
use tracing::warn;

#[allow(clippy::result_large_err)]
pub fn extract_protocol_header(headers: &HeaderMap) -> Result<String, Response> {
    if let Some(protocol_header) = headers.get("Sec-WebSocket-Protocol") {
        if let Ok(protocol_header_str) = protocol_header.to_str() {
            Ok(protocol_header_str.to_string())
        } else {
            warn!("Sec-WebSocket-Protocol was provided, but it could not be parsed, make sure the value is valid UTF-8",);
            Err(Response::builder().status(StatusCode::BAD_REQUEST).body(
                    "Sec-WebSocket-Protocol was provided, but it could not be parsed, make sure the value is valid UTF-8",
                ))
        }
    } else {
        warn!("Got a ocpp connection without Sec-WebSocket-Protocol header, this is not allowed since we can not select a ocpp protocol to use");
        Err(Response::builder().status(StatusCode::BAD_REQUEST).body(
            "Sec-WebSocket-Protocol was missing, it has to be either 'ocpp1.6' or 'ocpp2.0.1'",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::http::{HeaderMap, HeaderValue, StatusCode};

    #[test]
    fn test_header_present_and_valid() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Sec-WebSocket-Protocol",
            HeaderValue::from_static("ocpp1.6"),
        );
        let result = extract_protocol_header(&headers);
        assert_eq!(result.unwrap(), "ocpp1.6");
    }

    #[test]
    fn test_header_present_invalid_utf8() {
        let mut headers = HeaderMap::new();
        // 0xFF is not valid UTF-8
        let invalid = HeaderValue::from_bytes(b"\xFF\xFF\xFF").unwrap();
        headers.insert("Sec-WebSocket-Protocol", invalid);
        let result = extract_protocol_header(&headers);
        let err = result.unwrap_err();
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
        let body = futures::executor::block_on(err.into_body().into_string()).unwrap();
        assert!(body.contains("could not be parsed"));
    }

    #[test]
    fn test_header_missing() {
        let headers = HeaderMap::new();
        let result = extract_protocol_header(&headers);
        let err = result.unwrap_err();
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
        let body = futures::executor::block_on(err.into_body().into_string()).unwrap();
        assert!(body.contains("Sec-WebSocket-Protocol was missing"));
    }
}
