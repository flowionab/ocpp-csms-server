use crate::network_interface::json::validate_protocol::extract_protocol_header::extract_protocol_header;
use crate::network_interface::json::validate_protocol::select_protocol_and_handle_response::select_protocol_and_handle_response;
use crate::network_interface::OcppProtocol;
use poem::{http::HeaderMap, Response};

#[allow(clippy::result_large_err)]
pub fn validate_protocol(headers: &HeaderMap) -> Result<OcppProtocol, Response> {
    let protocol = extract_protocol_header(headers)?;
    let protocol = select_protocol_and_handle_response(&protocol)?;
    Ok(protocol.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::http::{HeaderMap, HeaderValue, StatusCode};

    #[test]
    fn test_valid_protocol() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Sec-WebSocket-Protocol",
            HeaderValue::from_static("ocpp1.6"),
        );
        let result = validate_protocol(&headers);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), OcppProtocol::Ocpp1_6);
    }

    #[test]
    fn test_invalid_protocol() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Sec-WebSocket-Protocol",
            HeaderValue::from_static("invalid"),
        );
        let result = validate_protocol(&headers);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
    }
}
