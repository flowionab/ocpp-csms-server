use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use poem::http::header::AUTHORIZATION;
use poem::http::{HeaderMap, HeaderValue, StatusCode};
use poem::Response;
use std::str::from_utf8;
use tracing::info;
use tracing::log::warn;

#[allow(clippy::result_large_err)]
pub fn extract_password(header_map: &HeaderMap) -> Result<Option<String>, Response> {
    match header_map.get(AUTHORIZATION) {
        None => {
            info!("No Authorization header provided");
            Ok(None)
        }
        Some(header) => {
            let value = parse_authorization_header_value(header)?;
            Ok(Some(value))
        }
    }
}

#[allow(clippy::result_large_err)]
pub fn parse_authorization_header_value(header: &HeaderValue) -> Result<String, Response> {
    let raw_header = extract_raw_value_from_header_value(header)?;
    let encoded = get_encoded_part(&raw_header)?;
    let decoded = decode_header_value(&encoded)?;
    let password = split_and_get_password(&decoded)?;
    Ok(password)
}

#[allow(clippy::result_large_err)]
pub fn get_encoded_part(value: &str) -> Result<String, Response> {
    let splits = value.split(" ").collect::<Vec<_>>();
    match splits.first() {
        None => {
            warn!("Authorization header was provided, but the content should start with Basic and have base64 encoded credentials");
            return Err(Response::builder().status(StatusCode::BAD_REQUEST).body(
                "Authorization header was provided, but the content should start with Basic and have base64 encoded credentials".to_string(),
            ));
        }
        Some(val) => {
            if *val != "Basic" {
                warn!("Authorization header was provided, but the only supported credential type is Basic");
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

#[allow(clippy::result_large_err)]
pub fn split_and_get_password(value: &str) -> Result<String, Response> {
    let splits = value.split(":").collect::<Vec<_>>();
    match splits.get(1) {
        None => {
            warn!("Authorization header was provided, but the content was not correct formatted, it should have <username>:<password> layout");
            Err(Response::builder().status(StatusCode::BAD_REQUEST).body(
                "Authorization header was provided, but the content was not correct formatted, it should have <username>:<password> layout".to_string(),
            ))
        }
        Some(part) => Ok(part.to_string()),
    }
}
