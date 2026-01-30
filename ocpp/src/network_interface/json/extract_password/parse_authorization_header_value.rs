use crate::network_interface::json::extract_password::decode_header_value::decode_header_value;
use crate::network_interface::json::extract_password::extract_raw_value_from_header_value::extract_raw_value_from_header_value;
use crate::network_interface::json::extract_password::get_encoded_part::get_encoded_part;
use crate::network_interface::json::extract_password::split_and_get_password::split_and_get_password;
use poem::Response;
use poem::http::HeaderValue;

#[allow(clippy::result_large_err)]
pub fn parse_authorization_header_value(header: &HeaderValue) -> Result<String, Response> {
    let raw_header = extract_raw_value_from_header_value(header)?;
    let encoded = get_encoded_part(&raw_header)?;
    let decoded = decode_header_value(&encoded)?;
    let password = split_and_get_password(&decoded)?;
    Ok(password)
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::http::HeaderValue;
    use poem::http::StatusCode;

    #[test]
    fn test_parse_valid_authorization_header() {
        let header = HeaderValue::from_static("Basic dXNlcjpwYXNz");
        let result = parse_authorization_header_value(&header);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "pass");
    }

    #[test]
    fn test_parse_invalid_authorization_header() {
        let header = HeaderValue::from_static("InvalidHeader");
        let result = parse_authorization_header_value(&header);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
    }
}
