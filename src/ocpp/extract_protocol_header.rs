use poem::{
    http::{HeaderMap, StatusCode},
    Response,
};
use tracing::{info, warn};

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
