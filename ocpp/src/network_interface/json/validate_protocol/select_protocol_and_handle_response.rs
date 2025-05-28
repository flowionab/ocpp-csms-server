use crate::network_interface::json::validate_protocol::select_protocol::select_protocol;
use poem::{http::StatusCode, Response};
use tracing::warn;

#[allow(clippy::result_large_err)]
pub fn select_protocol_and_handle_response(protocol: &str) -> Result<&'static str, Response> {
    match select_protocol(protocol) {
        Some(p) => Ok(p),
        None => {
            warn!(
                "None of the provided protocols was supported, provided options was {}",
                protocol
            );
            Err(Response::builder().status(StatusCode::BAD_REQUEST).body(
                    format!("Sec-WebSocket-Protocol was provided, but no protocol was supported by the server, it has to be either 'ocpp1.6' or 'ocpp2.0.1'. Options provided was {}", protocol),
                ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::http::StatusCode;

    #[test]
    fn test_supported_ocpp16() {
        let result = select_protocol_and_handle_response("ocpp1.6");
        assert_eq!(result.unwrap(), "ocpp1.6");
    }

    #[test]
    fn test_supported_ocpp201() {
        let result = select_protocol_and_handle_response("ocpp2.0.1");
        assert_eq!(result.unwrap(), "ocpp2.0.1");
    }

    #[test]
    fn test_unsupported_protocol() {
        let result = select_protocol_and_handle_response("foo");
        let err = result.unwrap_err();
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
        let body = futures::executor::block_on(err.into_body().into_string()).unwrap();
        assert!(body.contains("no protocol was supported"));
        assert!(body.contains("foo"));
    }

    #[test]
    fn test_multiple_protocols_first_supported() {
        let result = select_protocol_and_handle_response("ocpp2.0.1,ocpp1.6");
        assert_eq!(result.unwrap(), "ocpp2.0.1");
    }

    #[test]
    fn test_multiple_protocols_none_supported() {
        let result = select_protocol_and_handle_response("foo,bar");
        let err = result.unwrap_err();
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
        let body = futures::executor::block_on(err.into_body().into_string()).unwrap();
        assert!(body.contains("foo,bar"));
    }
}
