use crate::ocpp::extract_protocol_header::extract_protocol_header;
use crate::ocpp::ocpp_protocol::OcppProtocol;
use crate::ocpp::select_protocol_and_handle_response::select_protocol_and_handle_response;
use poem::{http::HeaderMap, Response};

pub fn validate_protocol(headers: &HeaderMap) -> Result<OcppProtocol, Response> {
    let protocol = extract_protocol_header(headers)?;
    let protocol = select_protocol_and_handle_response(&protocol)?;
    Ok(protocol.try_into().unwrap())
}
