use crate::ocpp::select_protocol::select_protocol;
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
